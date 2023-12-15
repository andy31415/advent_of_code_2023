#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Copy, Clone)]
enum Item {
    Free,
    Movable,
    Immovable,
}

impl From<char> for Item {
    fn from(value: char) -> Self {
        match value {
            '.' => Item::Free,
            'O' => Item::Movable,
            '#' => Item::Immovable,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Map {
    data: Vec<Vec<Item>>,
}

impl Map {
    fn swap(&mut self, a: (usize, usize), b: (usize, usize)) {
        (*self.at_mut(a), *self.at_mut(b)) = (self.at(b), self.at(a));
    }

    fn at(&self, pos: (usize, usize)) -> Item {
        *self
            .data
            .get(pos.0)
            .and_then(|v| v.get(pos.1))
            .expect("valid position")
    }

    fn at_mut(&mut self, pos: (usize, usize)) -> &mut Item {
        self.data
            .get_mut(pos.0)
            .and_then(|v| v.get_mut(pos.1))
            .expect("valid position")
    }

    fn rows(&self) -> usize {
        self.data.len()
    }

    fn cols(&self) -> usize {
        self.data.get(0).map(|v| v.len()).unwrap_or(0)
    }

    fn push_up(&mut self) {
        // VERY slow algorithm to push one space up each time
        // Given small data set this is ok. We could do N2 instead of N3 if we wanted,
        // but more work
        loop {
            let mut changes = 0;

            for r in 1..self.rows() {
                for c in 0..self.cols() {
                    let above = (r - 1, c);
                    if (self.at((r, c)) == Item::Movable) && (self.at(above) == Item::Free) {
                        self.swap((r, c), above);
                        changes += 1;
                    }
                }
            }

            if changes == 0 {
                break;
            }
        }
    }

    fn score_weight(&self) -> usize {
        let mut total = 0usize;

        for r in 0..self.rows() {
            for c in 0..self.rows() {
                if self.at((r, c)) == Item::Movable {
                    total += self.rows() - r;
                }
            }
        }

        total
    }
}

fn parse_map(input: &str) -> Map {
    Map {
        data: input
            .split('\n')
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect(),
    }
}

pub fn part1(input: &str) -> usize {
    let mut map = parse_map(input);
    map.push_up();
    map.score_weight()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../example.txt")), 136);
    }

    #[test]
    fn test_push_example() {
        let mut map = parse_map(include_str!("../example.txt"));
        map.push_up();

        assert_eq!(map, parse_map(include_str!("../example_pushed.txt")));
    }

    #[test]
    fn test_push_up() {
        let mut map = parse_map("#.O\n...\nOOO");

        map.push_up();

        assert_eq!(
            map,
            Map {
                data: vec![
                    vec![Item::Immovable, Item::Movable, Item::Movable],
                    vec![Item::Movable, Item::Free, Item::Movable],
                    vec![Item::Free, Item::Free, Item::Free],
                ],
            }
        );
    }

    #[test]
    fn test_swap() {
        let mut map = parse_map("#.O\nOOO\n..#");
        map.swap((0, 0), (2, 1));

        assert_eq!(
            map,
            Map {
                data: vec![
                    vec![Item::Free, Item::Free, Item::Movable],
                    vec![Item::Movable, Item::Movable, Item::Movable],
                    vec![Item::Free, Item::Immovable, Item::Immovable],
                ],
            }
        );
    }

    #[test]
    fn test_map_parse() {
        assert_eq!(
            parse_map("#.O\nOOO\n..#"),
            Map {
                data: vec![
                    vec![Item::Immovable, Item::Free, Item::Movable],
                    vec![Item::Movable, Item::Movable, Item::Movable],
                    vec![Item::Free, Item::Free, Item::Immovable],
                ],
            }
        );
    }
}
