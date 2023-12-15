use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Write},
};

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

#[derive(Debug, PartialEq, PartialOrd, Clone, Hash, Eq, Ord)]
struct Map {
    data: Vec<Vec<Item>>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.rows() {
            for c in 0..self.cols() {
                f.write_char(match self.at((r, c)) {
                    Item::Free => '.',
                    Item::Movable => 'O',
                    Item::Immovable => '#',
                })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
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

    fn move_pos(&self, pos: (usize, usize), dir: (i32, i32)) -> Option<(usize, usize)> {
        let test_r = pos.0 as i32 + dir.0;
        if test_r < 0 || test_r >= self.rows() as i32 {
            return None;
        }

        let test_c = pos.1 as i32 + dir.1;
        if test_c < 0 || test_c >= self.cols() as i32 {
            return None;
        }

        Some((test_r as usize, test_c as usize))
    }

    fn push(&mut self, dir: (i32, i32)) {
        // Somewhat slow algorithm to push one space up each time
        let row_range: Vec<usize> = match dir.0 {
            -1 => (1..self.rows()).collect(),
            0 => (0..self.rows()).collect(),
            1 => (0..(self.rows() - 1)).rev().collect(),
            _ => unreachable!(),
        };

        let col_range: Vec<usize> = match dir.1 {
            -1 => (1..self.cols()).collect(),
            0 => (0..self.cols()).collect(),
            1 => (0..(self.cols() - 1)).rev().collect(),
            _ => unreachable!(),
        };

        for r in row_range {
            for c in col_range.as_slice() {
                let mut current = (r as usize, *c as usize);
                let mut other = self.move_pos(current, dir).expect("valid");
                if self.at(current) != Item::Movable {
                    continue;
                }

                if self.at(other) != Item::Free {
                    continue;
                }

                // keep moving while we can
                loop {
                    self.swap(current, other);

                    current = other;
                    other = match self.move_pos(current, dir) {
                        Some(n) => n,
                        None => break,
                    };

                    if self.at(other) != Item::Free {
                        break;
                    }
                }
            }
        }
    }

    fn push_up(&mut self) {
        self.push((-1, 0));
    }

    fn cycle(&mut self) {
        self.push((-1, 0));
        self.push((0, -1));
        self.push((1, 0));
        self.push((0, 1));
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

pub fn part2(input: &str, cnt: usize) -> usize {
    let mut map = parse_map(input);

    let dirs = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];

    // do one cycle to start in a maybe-stable position
    let mut rotations = 0;
    let mut options = HashSet::new();

    while rotations < cnt {
        map.cycle();
        rotations += 1;

        if options.contains(&map) {
            break;
        }
        options.insert(map.clone());
    }

    let target = map.clone();
    let mut cycle_size = 0usize;
    loop {
        map.cycle();
        cycle_size += 1;
        rotations += 1;
        if map == target {
            break;
        }
    }

    let left = cnt - rotations;
    let left = left % cycle_size;
    for _ in 0..left {
        for dir in dirs.iter() {
            map.push(*dir);
        }
    }

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
    fn test_part2() {
        assert_eq!(part2(include_str!("../example.txt"), 1000000000), 64);
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
