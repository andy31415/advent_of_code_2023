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

#[cfg(test)]
mod tests {
    use crate::{ItemType, PartItem, PartItemIterator};

    fn get_example_schematic() -> &'static str {
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .trim()
    }

    #[test]
    fn parse_example() {
        assert_eq!(
            PartItemIterator::new(get_example_schematic()),
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
