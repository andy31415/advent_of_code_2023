use std::{
    collections::{HashMap, VecDeque},
    fmt::{Debug, Write},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::value,
    multi::{many1, separated_list1},
    IResult, Parser,
};
use tracing::debug;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
enum MapPoint {
    Ground,
    Pipe(Direction, Direction),
    Start,
}

impl MapPoint {
    fn graphic_char(&self) -> char {
        match self {
            MapPoint::Ground => '.',
            MapPoint::Start => 'S',
            x => {
                if x.has_connection(Direction::Left) {
                    if x.has_connection(Direction::Right) {
                        '─'
                    } else if x.has_connection(Direction::Up) {
                        '┘'
                    } else if x.has_connection(Direction::Down) {
                        '┐'
                    } else {
                        '?'
                    }
                } else if x.has_connection(Direction::Right) {
                    if x.has_connection(Direction::Up) {
                        '└'
                    } else if x.has_connection(Direction::Down) {
                        '┌'
                    } else {
                        '?'
                    }
                } else if x.has_connection(Direction::Up) && x.has_connection(Direction::Down) {
                    '│'
                } else {
                    '?'
                }
            }
        }
    }

    fn left_of(&self, other: MapPoint) -> bool {
        self.has_connection(Direction::Right) && other.has_connection(Direction::Left)
    }

    fn right_of(&self, other: MapPoint) -> bool {
        self.has_connection(Direction::Left) && other.has_connection(Direction::Right)
    }

    fn above(&self, other: MapPoint) -> bool {
        self.has_connection(Direction::Down) && other.has_connection(Direction::Up)
    }

    fn below(&self, other: MapPoint) -> bool {
        self.has_connection(Direction::Up) && other.has_connection(Direction::Down)
    }

    fn has_connection(&self, d: Direction) -> bool {
        match self {
            MapPoint::Start => true,
            MapPoint::Pipe(_, x) | MapPoint::Pipe(x, _) if *x == d => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, PartialOrd, Clone)]
struct Line {
    points: Vec<MapPoint>,
}

impl Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("L[")?;
        for x in self.points.iter() {
            f.write_char(x.graphic_char())?;
        }
        f.write_str("]")
    }
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    many1(alt((
        value(MapPoint::Ground, tag(".")),
        value(MapPoint::Pipe(Direction::Left, Direction::Right), tag("-")),
        value(MapPoint::Pipe(Direction::Up, Direction::Down), tag("|")),
        value(MapPoint::Pipe(Direction::Up, Direction::Right), tag("L")),
        value(MapPoint::Pipe(Direction::Up, Direction::Left), tag("J")),
        value(MapPoint::Pipe(Direction::Down, Direction::Left), tag("7")),
        value(MapPoint::Pipe(Direction::Down, Direction::Right), tag("F")),
        value(MapPoint::Start, tag("S")),
    )))
    .map(|points| Line { points })
    .parse(input)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct Point {
    row: usize,
    col: usize,
}

#[derive(PartialEq, PartialOrd, Clone)]
struct Map {
    lines: Vec<Line>,
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("MAP: {\n")?;
        for l in self.lines.iter() {
            f.write_fmt(format_args!("  {:?}\n", l))?;
        }
        f.write_str("}\n")
    }
}

impl Map {
    fn at(&self, p: Point) -> Option<MapPoint> {
        self.lines.get(p.row)?.points.get(p.col).copied()
    }

    fn left(&self, p: Point) -> Option<Point> {
        if p.col > 0 {
            Some(Point {
                col: p.col - 1,
                ..p
            })
        } else {
            None
        }
    }

    fn right(&self, p: Point) -> Option<Point> {
        if p.col + 1 < self.lines.first()?.points.len() {
            Some(Point {
                col: p.col + 1,
                ..p
            })
        } else {
            None
        }
    }

    fn up(&self, p: Point) -> Option<Point> {
        if p.row > 0 {
            Some(Point {
                row: p.row - 1,
                ..p
            })
        } else {
            None
        }
    }

    fn down(&self, p: Point) -> Option<Point> {
        if p.row + 1 < self.lines.len() {
            Some(Point {
                row: p.row + 1,
                ..p
            })
        } else {
            None
        }
    }

    fn neighbours(&self, point: Point) -> impl Iterator<Item = Point> {
        let mut result = Vec::new();

        let value = match self.at(point) {
            Some(v) => v,
            None => return result.into_iter(),
        };

        if let Some(x) = self.left(point) {
            if let Some(r) = self.at(x) {
                if value.right_of(r) {
                    result.push(x);
                }
            }
        }

        if let Some(x) = self.right(point) {
            if let Some(r) = self.at(x) {
                if value.left_of(r) {
                    result.push(x);
                }
            }
        }

        if let Some(x) = self.up(point) {
            if let Some(r) = self.at(x) {
                if value.below(r) {
                    result.push(x);
                }
            }
        }

        if let Some(x) = self.down(point) {
            if let Some(r) = self.at(x) {
                if value.above(r) {
                    result.push(x);
                }
            }
        }

        result.into_iter()
    }

    fn start_point(&self) -> Option<Point> {
        for (row, line) in self.lines.iter().enumerate() {
            for (col, item) in line.points.iter().enumerate() {
                if *item == MapPoint::Start {
                    return Some(Point { row, col });
                }
            }
        }
        None
    }

    fn in_loop(&self, p: Point) -> bool {
        // a point is in a loop if it has a value and exactly two neighbours
        self.neighbours(p).count() == 2
    }

    fn distances(&self) -> HashMap<Point, u32> {
        let mut processing = VecDeque::new();
        let mut processed = HashMap::new();

        processing.push_back((
            self.start_point()
                .expect("valid input should have a start point"),
            0u32,
        ));

        while let Some((point, value)) = processing.pop_front() {
            if processed.contains_key(&point) {
                continue;
            }
            tracing::debug!("Processing {:?} value {}", point, value);

            for n in self.neighbours(point) {
                tracing::debug!("  => Neighbor {:?}", n);

                processing.push_back((n, value + 1));
            }

            processed.insert(point, value);
        }
        processed
    }

    fn as_loop_only(&self) -> Map {
        let distances = self.distances();

        Map {
            lines: self
                .lines
                .iter()
                .enumerate()
                .map(|(row, line)| Line {
                    points: line
                        .points
                        .iter()
                        .enumerate()
                        .map(|(col, p)| {
                            if distances.contains_key(&Point { row, col }) {
                                *p
                            } else {
                                MapPoint::Ground
                            }
                        })
                        .collect(),
                })
                .collect(),
        }
    }

    #[tracing::instrument(skip(self))]
    pub fn inside_outside(&self) -> u32 {
        // only things in the main loop will be relevant
        let distances = self.distances();

        let total = self
            .lines
            .iter()
            .enumerate()
            .map(|(row, line)| {
                // logic:
                //   paritition scan for lines:
                //   odd up/down we are inside, even up/down we are outside
                let mut up = false;
                let mut down = false;
                let mut inside = 0u32;

                debug!("Checking line {:?}", line);

                for (col, p) in line.points.iter().enumerate() {
                    if distances.contains_key(&Point { row, col }) {
                        debug!("Contains: {},{}", row, col);
                        if *p == MapPoint::Start {
                            debug!("   DEBUG start point: {},{}", row, col);
                            // FIXME: now what? Figure out where to start
                            for n in self.neighbours(Point { row, col }) {
                                if self.at(n).expect("ok").above(*p) {
                                    debug!("    ABOVE");
                                    up = !up;
                                }
                                if self.at(n).expect("ok").below(*p) {
                                    debug!("    BELOW");
                                    down = !down;
                                }
                            }
                        } else {
                            if p.has_connection(Direction::Down) {
                                down = !down;
                            }
                            if p.has_connection(Direction::Up) {
                                up = !up;
                            }
                        }
                    } else if up && down {
                        debug!("Add inside: {},{}", row, col);
                        inside += 1;
                    } 
                }
                debug!("  Inside: {}", inside);
                inside
            })
            .sum();

        total
    }
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    separated_list1(line_ending, parse_line)
        .map(|lines| Map { lines })
        .parse(input)
}

pub fn part1(input: &str) -> u32 {
    let (r, map) = parse_map(input).expect("valid input");
    assert_eq!(r, "");

    let distances = map.distances();

    distances
        .iter()
        .filter(|(k, _)| map.in_loop(**k))
        .map(|(_, v)| *v)
        .max()
        .expect("some data")
}

pub fn part2(input: &str) -> u32 {
    let (r, map) = parse_map(input).expect("valid input");
    assert_eq!(r, "");

    let map = map.as_loop_only();

    tracing::info!("{:?}", &map);

    map.inside_outside()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../example1.txt")), 4);
        assert_eq!(part1(include_str!("../example2.txt")), 8);
    }

    #[test_log::test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../example_inside_outside_1.txt")), 4);
        assert_eq!(part2(include_str!("../example_inside_outside_2.txt")), 8);
        assert_eq!(part2(include_str!("../example_inside_outside_3.txt")), 10);
    }
}
