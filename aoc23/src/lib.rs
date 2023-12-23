use std::collections::{HashMap, HashSet};

use pathfinding::directed::dijkstra::dijkstra;
use tracing::{info, instrument, trace};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn all() -> [Direction; 4] {
        [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
    }

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

impl std::ops::Add<Direction> for Point {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        self + rhs.vec()
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

struct JunctionGraph {
    distances: HashMap<Point, Vec<(Point, usize)>>,
}

impl JunctionGraph {
    fn max_distance(&self, start: Point, end: Point) -> usize {
        // Terrible algorighm, however since few junctions maybe it works
        // on these maps ...
        self.max_distance_rec(start, 0, end, &mut HashSet::new())
    }

    #[instrument(skip_all)]
    fn max_distance_rec(
        &self,
        start: Point,
        so_far: usize,
        end: Point,
        visited: &mut HashSet<Point>,
    ) -> usize {
        trace!("{:?} distance {}", start, so_far);
        let neighbours = match self.distances.get(&start) {
            Some(v) => v,
            None => return 0,
        };

        let mut m = so_far;

        for (n, d) in neighbours.iter().filter(|(n, _)| !visited.contains(n)).collect::<Vec<_>>() {
            if *n == end {
                m = m.max(so_far + d)
            } else {
                visited.insert(*n);
                m = m.max(self.max_distance_rec(*n, so_far + d, end, visited));
                visited.remove(n);
            }
        }
        m
    }
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

    fn no_slopes(&self) -> Self {
        let mut data = self.data.clone();
        for (_, v) in data.iter_mut() {
            if matches!(v, Cell::Slope(_)) {
                *v = Cell::Empty;
            }
        }

        Self {
            data,
            rows: self.rows,
            cols: self.cols,
        }
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
        // Nodes are start, end and any junction
        let mut junctions = self
            .data
            .iter()
            .filter(|(p, v)| {
                **v != Cell::Wall
                    && p.row > 0
                    && p.col > 0
                    && p.row + 1 < self.rows as i32
                    && p.col + 1 < self.cols as i32
            })
            .map(|(p, _)| *p)
            .filter(|p| self.is_junction(*p))
            .collect::<HashSet<_>>();
        junctions.insert(start);
        junctions.insert(end);
        info!("Junctions: {:?}", junctions);

        let mut distances: HashMap<Point, Vec<(Point, usize)>> = HashMap::new();

        // assume this is a graph. Figure out the length of a DIRECT path from each
        // junction to another junction
        for a in junctions.iter() {
            for b in junctions.iter().filter(|x| *x != a) {
                if let Some(r) = dijkstra(
                    a,
                    |x| {
                        Direction::all()
                            .iter()
                            .filter(|d| self.allow(*x, **d))
                            .map(|d| *x + *d)
                            .filter(|p| p == a || p == b || !junctions.contains(p))
                            .map(|p| (p, 1usize))
                            .collect::<Vec<_>>()
                    },
                    |p| p == b,
                ) {
                    trace!("TODO: path from {:?} to {:?} == {}", a, b, r.1);
                    match distances.get_mut(a) {
                        Some(v) => v.push((*b, r.1)),
                        None => {
                            distances.insert(*a, vec![(*b, r.1)]);
                        }
                    }
                }
            }
        }
        
        let g = JunctionGraph {
            distances,
        };

        g.max_distance(start, end)
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
    let input = Input::parse(input).no_slopes();
    input.longest_path(
        (0, 1).into(),
        ((input.rows - 1) as i32, (input.cols - 2) as i32).into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../example.txt")), 94);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../example.txt")), 154);
    }
}
