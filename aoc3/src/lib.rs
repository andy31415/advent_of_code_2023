use std::{fmt::Debug, str::Chars};

#[derive(Debug, Copy, Clone, PartialEq)]
enum ItemType {
    Symbol(char),
    PartNumber(u32),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PartItem {
    item_type: ItemType,
    line: u32,
    col: u32,
    len: u32,
}

impl PartItem {
    pub fn is_adjacent_part_number(&self, symbol: &PartItem) -> bool {
        // we only consider symbols
        assert!(matches!(symbol.item_type, ItemType::Symbol(_)));
        assert!(symbol.len == 1);

        // adjacent to part numbers
        assert!(matches!(self.item_type, ItemType::PartNumber(_)));

        // Positions have to be in the entire range around the part item
        // First line has to match +-1
        if (symbol.line + 1 < self.line) || (symbol.line > self.line + 1) {
            return false;
        }

        // should be within range
        (symbol.col + 1 >= self.col) && (symbol.col <= self.col + self.len)
    }
}

#[derive(Clone)]
pub struct PartItemIterator<'a> {
    rest: std::iter::Peekable<Chars<'a>>,
    line: u32,
    col: u32,
}

impl<'a> Debug for PartItemIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for item in self.clone() {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", item)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<'a> PartItemIterator<'a> {
    pub fn new(data: &'a str) -> Self {
        Self {
            rest: data.chars().peekable(),
            line: 0,
            col: 0,
        }
    }
}

impl<'a> Iterator for PartItemIterator<'a> {
    type Item = PartItem;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.rest.next();
            self.col += 1;
            match next {
                Some('.') => {}
                Some('\n') => {
                    self.line += 1;
                    self.col = 0;
                }
                Some(num) if num.is_ascii_digit() => {
                    let mut part = PartItem {
                        item_type: ItemType::PartNumber(0), // item type will be set later
                        line: self.line,
                        col: self.col - 1,
                        len: 1,
                    };

                    let mut code = vec![num];
                    loop {
                        match self.rest.peek() {
                            Some('0'..='9') => {
                                self.col += 1;
                                part.len += 1;
                                code.push(self.rest.next().unwrap());
                            }
                            _ => {
                                // we know part number is valid
                                part.item_type = ItemType::PartNumber(
                                    String::from_iter(code).parse::<u32>().unwrap(),
                                );
                                return Some(part);
                            }
                        }
                    }
                }
                Some(symbol) => {
                    return Some(PartItem {
                        item_type: ItemType::Symbol(symbol),
                        line: self.line,
                        len: 1,
                        col: self.col - 1,
                    })
                }
                None => return None,
            }
        }
    }
}

impl<'a, I> PartialEq<I> for PartItemIterator<'a>
where
    I: IntoIterator<Item = PartItem> + Clone,
{
    fn eq(&self, other: &I) -> bool {
        let mut ia = self.clone();
        let mut ib = other.clone().into_iter();
        loop {
            match (ia.next(), ib.next()) {
                (None, None) => return true,
                (None, Some(_)) => return false,
                (Some(_), None) => return false,
                (Some(a), Some(b)) => {
                    if a != b {
                        return false;
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Part {
    pub number: u32,
    pub symbol: char,
}

pub fn parts(input: &str) -> Vec<Part> {
    let (symbols, numbers): (Vec<_>, Vec<_>) = PartItemIterator::new(input)
        .partition(|part| matches!(part.item_type, ItemType::Symbol(_)));

    let mut result = Vec::new();

    for n in numbers {
        let s = symbols
            .iter()
            .filter(|s| n.is_adjacent_part_number(s))
            .collect::<Vec<_>>();

        match s.len() {
            0 => {}
            1 => match (n, s.get(0)) {
                (
                    PartItem {
                        item_type: ItemType::PartNumber(number),
                        ..
                    },
                    Some(PartItem {
                        item_type: ItemType::Symbol(symbol),
                        ..
                    }),
                ) => result.push(Part {
                    number,
                    symbol: *symbol,
                }),
                _ => panic!("Unexpected state"),
            },
            _ => panic!("Multiple symbols for a single part {:?}: {:#?}!", n, s),
        }
    }
    result
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Gear {
    pub n1: u32,
    pub n2: u32,
}

impl Gear {
    pub fn ratio(&self) -> u32 {
        self.n1 * self.n2
    }
}

pub fn gears(input: &str) -> Vec<Gear> {
    let (symbols, numbers): (Vec<_>, Vec<_>) = PartItemIterator::new(input)
        .partition(|part| matches!(part.item_type, ItemType::Symbol(_)));

    let mut result = Vec::new();

    for s in symbols
        .iter()
        .filter(|s| s.item_type == ItemType::Symbol('*'))
    {
        // Find all numbers that are associated to this symbol
        let n = numbers
            .iter()
            .filter_map(|n| {
                if !n.is_adjacent_part_number(s) {
                    return None;
                }
                match n.item_type {
                    ItemType::PartNumber(n) => Some(n),
                    _ => panic!("expecting only part numbers"),
                }
            })
            .collect::<Vec<_>>();

        if n.len() == 2 {
            result.push(Gear {
                n1: *n.first().unwrap(),
                n2: *n.last().unwrap(),
            })
        }
    }
    result
}

pub fn part_1_sum_parts(input: &str) -> u32 {
    parts(input).iter().map(|p| p.number).sum()
}

pub fn part_2_sum_gear_ratios(input: &str) -> u32 {
    gears(input).iter().map(|g| g.ratio()).sum()
}

//////// Totaly alternate implementation
pub struct Board {
    lines: Vec<Vec<char>>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SymbolPos {
    pub symbol: char,
    pub line: usize,
    pub col: usize,
}

impl Board {
    pub fn new(data: &str) -> Board {
        Board {
            lines: data.split('\n').map(|l| l.chars().collect()).collect(),
        }
    }

    pub fn symbols(&self) -> Vec<SymbolPos> {
        self.lines
            .iter()
            .enumerate()
            .flat_map(|(line, data)| {
                data.iter().enumerate().filter_map(move |(col, ch)| {
                    if ch.is_ascii_digit() || *ch == '.' {
                        None
                    } else {
                        Some(SymbolPos {
                            symbol: *ch,
                            line,
                            col,
                        })
                    }
                })
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_symbols() {
        assert_eq!(
            Board::new(include_str!("../example.txt")).symbols(),
            [
                SymbolPos {
                    symbol: '*',
                    line: 1,
                    col: 3
                },
                SymbolPos {
                    symbol: '#',
                    line: 3,
                    col: 6
                },
                SymbolPos {
                    symbol: '*',
                    line: 4,
                    col: 3
                },
                SymbolPos {
                    symbol: '+',
                    line: 5,
                    col: 5
                },
                SymbolPos {
                    symbol: '$',
                    line: 8,
                    col: 3
                },
                SymbolPos {
                    symbol: '*',
                    line: 8,
                    col: 5
                },
            ]
        );
    }

    #[test]
    fn test_gears() {
        assert_eq!(
            gears(include_str!("../example.txt")),
            [Gear { n1: 467, n2: 35 }, Gear { n1: 755, n2: 598 }]
        );

        assert_eq!(
            gears(include_str!("../example.txt"))
                .iter()
                .map(|g| g.ratio())
                .sum::<u32>(),
            467835
        );
    }

    #[test]
    fn test_adjacency() {
        let (symbols, numbers): (Vec<_>, Vec<_>) =
            PartItemIterator::new(include_str!("../example.txt"))
                .partition(|part| matches!(part.item_type, ItemType::Symbol(_)));

        // find all part numbers without a symbol
        assert_eq!(
            numbers
                .iter()
                .filter(|n| !symbols.iter().any(|s| n.is_adjacent_part_number(s)))
                .map(|p| p.item_type)
                .collect::<Vec<_>>(),
            [ItemType::PartNumber(114), ItemType::PartNumber(58),]
        );

        // sum all parts wit h a symbol
        assert_eq!(
            numbers
                .iter()
                .filter(|n| symbols.iter().any(|s| n.is_adjacent_part_number(s)))
                .map(|p| match p.item_type {
                    ItemType::PartNumber(n) => n,
                    _ => panic!("Should only have part numbers here"),
                })
                .sum::<u32>(),
            4361
        );

        assert_eq!(part_1_sum_parts(include_str!("../example.txt")), 4361);
    }

    #[test]
    fn parse_example() {
        assert_eq!(
            PartItemIterator::new(include_str!("../example.txt")),
            [
                PartItem {
                    line: 0,
                    col: 0,
                    len: 3,
                    item_type: ItemType::PartNumber(467)
                },
                PartItem {
                    line: 0,
                    col: 5,
                    len: 3,
                    item_type: ItemType::PartNumber(114)
                },
                PartItem {
                    line: 1,
                    col: 3,
                    len: 1,
                    item_type: ItemType::Symbol('*')
                },
                PartItem {
                    line: 2,
                    col: 2,
                    len: 2,
                    item_type: ItemType::PartNumber(35)
                },
                PartItem {
                    line: 2,
                    col: 6,
                    len: 3,
                    item_type: ItemType::PartNumber(633)
                },
                PartItem {
                    line: 3,
                    col: 6,
                    len: 1,
                    item_type: ItemType::Symbol('#')
                },
                PartItem {
                    line: 4,
                    col: 0,
                    len: 3,
                    item_type: ItemType::PartNumber(617)
                },
                PartItem {
                    line: 4,
                    col: 3,
                    len: 1,
                    item_type: ItemType::Symbol('*')
                },
                PartItem {
                    item_type: ItemType::Symbol('+'),
                    line: 5,
                    col: 5,
                    len: 1
                },
                PartItem {
                    item_type: ItemType::PartNumber(58),
                    line: 5,
                    col: 7,
                    len: 2
                },
                PartItem {
                    item_type: ItemType::PartNumber(592),
                    line: 6,
                    col: 2,
                    len: 3
                },
                PartItem {
                    item_type: ItemType::PartNumber(755),
                    line: 7,
                    col: 6,
                    len: 3
                },
                PartItem {
                    item_type: ItemType::Symbol('$'),
                    line: 8,
                    col: 3,
                    len: 1
                },
                PartItem {
                    item_type: ItemType::Symbol('*'),
                    line: 8,
                    col: 5,
                    len: 1
                },
                PartItem {
                    item_type: ItemType::PartNumber(664),
                    line: 9,
                    col: 1,
                    len: 3
                },
                PartItem {
                    item_type: ItemType::PartNumber(598),
                    line: 9,
                    col: 5,
                    len: 3
                }
            ]
        );
    }

    #[test]
    fn parse_symbols() {
        assert_eq!(
            PartItemIterator::new("@"),
            [PartItem {
                item_type: ItemType::Symbol('@'),
                line: 0,
                len: 1,
                col: 0,
            }]
        );
        assert_eq!(
            PartItemIterator::new("@x"),
            [
                PartItem {
                    item_type: ItemType::Symbol('@'),
                    line: 0,
                    len: 1,
                    col: 0,
                },
                PartItem {
                    item_type: ItemType::Symbol('x'),
                    line: 0,
                    len: 1,
                    col: 1,
                },
            ]
        );
        assert_eq!(
            PartItemIterator::new("..@..\n.x"),
            [
                PartItem {
                    item_type: ItemType::Symbol('@'),
                    line: 0,
                    len: 1,
                    col: 2,
                },
                PartItem {
                    item_type: ItemType::Symbol('x'),
                    line: 1,
                    len: 1,
                    col: 1,
                },
            ]
        );
    }

    #[test]
    fn adjacent_test() {
        let n = PartItem {
            item_type: ItemType::PartNumber(123),
            len: 3,
            line: 10,
            col: 10,
        };

        let sym = |line: u32, col: u32| PartItem {
            item_type: ItemType::Symbol('x'),
            len: 1,
            line,
            col,
        };

        for line in 9..=11 {
            for col in 9..=13 {
                assert!(n.is_adjacent_part_number(&sym(line, col)));
            }
            for col in 0..=8 {
                assert!(!n.is_adjacent_part_number(&sym(line, col)));
            }
            for col in 14..=20 {
                assert!(!n.is_adjacent_part_number(&sym(line, col)));
            }
        }

        for line in 0..=8 {
            for col in 0..=20 {
                assert!(!n.is_adjacent_part_number(&sym(line, col)));
            }
        }

        for line in 12..=20 {
            for col in 0..=20 {
                assert!(!n.is_adjacent_part_number(&sym(line, col)));
            }
        }
    }

    #[test]
    fn parse_parts() {
        assert!(PartItemIterator::new("").eq([]));

        assert!(PartItemIterator::new("123").eq([PartItem {
            item_type: ItemType::PartNumber(123),
            line: 0,
            len: 3,
            col: 0,
        }]));

        assert!(PartItemIterator::new("123.22").eq([
            PartItem {
                item_type: ItemType::PartNumber(123),
                line: 0,
                col: 0,
                len: 3,
            },
            PartItem {
                item_type: ItemType::PartNumber(22),
                line: 0,
                col: 4,
                len: 2,
            },
        ]));

        assert!(PartItemIterator::new("123\n..22").eq([
            PartItem {
                item_type: ItemType::PartNumber(123),
                line: 0,
                col: 0,
                len: 3,
            },
            PartItem {
                item_type: ItemType::PartNumber(22),
                line: 1,
                col: 2,
                len: 2,
            },
        ]));
    }
}
