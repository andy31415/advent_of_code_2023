use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn inverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    fn vec(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Cell {
    Empty,
    Wall,
    Slope(Direction),
}

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd, Eq, Ord, Hash)]
struct Point {
    row: i32,
    col: i32,
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl std::ops::Add<(i32, i32)> for Point {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self {
            row: self.row + rhs.0,
            col: self.col + rhs.1,
        }
    }
}

impl From<(i32, i32)> for Point {
    fn from((row, col): (i32, i32)) -> Self {
        Self { row, col }
    }
}

struct Input {
    data: HashMap<Point, Cell>,
    rows: usize,
    cols: usize,
}

impl Input {
    fn parse(input: &str) -> Self {
        let mut data = HashMap::with_capacity(input.len());
        let mut rows = 0;
        let mut cols = 0;

        for (row, l) in input.split("\n").enumerate() {
            rows += 1;
            cols = l.len();
            for (col, c) in l.chars().enumerate() {
                data.insert(
                    (row as i32, col as i32).into(),
                    match c {
                        '.' => Cell::Empty,
                        '#' => Cell::Wall,
                        '^' => Cell::Slope(Direction::North),
                        '>' => Cell::Slope(Direction::East),
                        'v' => Cell::Slope(Direction::South),
                        '<' => Cell::Slope(Direction::West),
                        _ => panic!("invalid input: {}", c),
                    },
                );
            }
        }
        Self { data, rows, cols }
    }

    /// Allow going from [p] towards direction [d]
    fn allow(&self, p: Point, d: Direction) -> bool {
        let c = *match self.data.get(&p) {
            Some(v) => v,
            None => return false,
        };

        let o = p + d.vec();

        // figure out where one could go... if it is a valid space,
        // then allow it
        match self.data.get(&o) {
            Some(Cell::Wall) => return false,
            None => return false,
            Some(_) => (),
        };

        c == Cell::Empty || c == Cell::Slope(d)
    }

    #[allow(dead_code)]
    fn is_junction(&self, p: Point) -> bool {
        let mut cnt = 0;

        if p == (5, 3).into() {
            eprintln!("P: {:?}", p);
        }

        for d in [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ] {
            let o = p + d.inverse().vec();

            cnt += match self.data.get(&o) {
                Some(Cell::Wall) => 0,
                None => 0,
                _ => 1,
            };
        }

        cnt > 2
    }

    fn longest_path(&self, start: Point, end: Point) -> usize {
        /*
        // Nodes are start, end and any junction
        let mut nodes = Vec::new();
        nodes.push(start);
        nodes.push(end);

        nodes.append(
            &mut self
                .data
                .iter()
                .filter(|(p, v)| {
                    **v != Cell::Wall
                        && p.row > 0
                        && p.col > 0
                        && p.row + 1 < self.rows as i32
                        && p.col + 1 < self.cols as i32
                })
                .map(|(p, v)| *p)
                .filter(|p| self.is_junction(*p))
                .collect::<Vec<_>>(),
        );
        */
        
        // This makes a STRONG assumption that there are no loops and 
        // the entire map is directed ... otherwise this is NP-complete ...
        let mut distances = HashMap::new();
        
        let mut q = VecDeque::new();
        q.push_back((start, Direction::South, 0 as usize));
        
        while let Some((p, entry, distance)) = q.pop_front() {
            match distances.get_mut(&p) {
                Some(old_distance) if *old_distance < distance => {
                    *old_distance = distance;
                    q.push_back((p, entry, distance)); // new top distance, find again
                }
                None => {
                    distances.insert(p, distance);
                }
                _ => (),
            };
            
            for d in [Direction::East, Direction::South, Direction::West, Direction::North] {
                if d == entry.inverse() {
                    continue
                }
                if self.allow(p, d) {
                    q.push_back((p + d.vec(), d, distance + 1));
                }
            }
        }
        *distances.get(&end).expect("has path to end")
    }
}

pub fn part1(input: &str) -> usize {
    let input = Input::parse(input);
    input.longest_path(
        (0, 1).into(),
        ((input.rows - 1) as i32, (input.cols - 2) as i32).into(),
    )
}

pub fn part2(input: &str) -> usize {
    // TODO: implement
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../example.txt")), 94);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../example.txt")), 0);
    }
}
