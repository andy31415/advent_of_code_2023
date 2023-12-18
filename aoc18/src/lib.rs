use std::{
    collections::{BTreeMap, HashSet},
    fmt::{Debug, Display, Write},
    hash::Hash,
    ops::Add,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::line_ending,
    combinator::value,
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult, Parser,
};
use nom_supreme::ParserExt;
use tracing::{info, instrument, trace};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn tuple(&self) -> (i64, i64) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

impl Add<(i64, i64)> for Direction {
    type Output = (i64, i64);

    fn add(self, rhs: (i64, i64)) -> Self::Output {
        let t = self.tuple();
        (rhs.0 + t.0, rhs.1 + t.1)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct DigInstruction<'a> {
    direction: Direction,
    distance: i64, // always positive, but easier math
    color: &'a str,
}

impl<'a> DigInstruction<'a> {
    fn color_to_distance(&self) -> Self {
        // COLOR is hex:
        let (col, dir) = self.color.split_at(self.color.len() - 1);
        
        Self {
            direction: match dir {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                _ => panic!("INVALID: {:?}", self),
            },
            distance: i64::from_str_radix(col, 16).expect("valid"),
            color: "",
        }
    }
}

struct DigMap<'a> {
    // locations of holes
    holes: BTreeMap<(i64, i64), &'a str>, // Color
    row_range: (i64, i64),                // upper range is exclusive
    col_range: (i64, i64),                // upper range is exclusive

    // digger position
    digger_pos: (i64, i64),
}

impl<'a> DigMap<'a> {
    fn new() -> Self {
        let mut holes = BTreeMap::new();
        let digger_pos = (0, 0);
        holes.insert(digger_pos, "");

        Self {
            holes,
            row_range: (0, 1),
            col_range: (0, 1),
            digger_pos,
        }
    }

    fn perform_instructions(&mut self, instructions: &[DigInstruction<'a>]) {
        for instruction in instructions {
            for _ in 0..instruction.distance {
                self.digger_pos = instruction.direction + self.digger_pos;
                self.holes.insert(self.digger_pos, instruction.color);

                if self.row_range.0 > self.digger_pos.0 {
                    self.row_range.0 = self.digger_pos.0;
                }
                if self.row_range.1 <= self.digger_pos.0 {
                    self.row_range.1 = self.digger_pos.0 + 1;
                }

                if self.col_range.0 > self.digger_pos.1 {
                    self.col_range.0 = self.digger_pos.1;
                }
                if self.col_range.1 <= self.digger_pos.1 {
                    self.col_range.1 = self.digger_pos.1 + 1;
                }
            }
        }
    }

    fn hole_at(&self, p: (i64, i64)) -> bool {
        self.holes.contains_key(&p)
    }

    fn find_inside(&self) -> (i64, i64) {
        for row in self.row_range.0..self.row_range.1 {
            for col in self.col_range.0..self.col_range.1 {
                let p = (row, col);

                if !self.hole_at(Direction::Left + p)
                    && self.hole_at(p)
                    && !self.hole_at(Direction::Right + p)
                {
                    return Direction::Right + p;
                }
            }
        }
        panic!("If all is stairs, this is not implemented");
    }

    fn flood_fill_inside(&mut self) {
        let mut fills = Vec::new();
        let mut seen = HashSet::new();
        fills.push(self.find_inside());

        while let Some(p) = fills.pop() {
            seen.insert(p);

            for d in [
                Direction::Left,
                Direction::Right,
                Direction::Up,
                Direction::Down,
            ] {
                let other = d + p;
                if self.hole_at(other) {
                    continue;
                }
                if !seen.contains(&other) {
                    fills.push(other);
                }
            }
        }

        for p in seen {
            self.holes.insert(p, "");
        }
    }

    fn dug_out_depth(&self) -> usize {
        self.holes.len()
    }
}

impl<'a> Display for DigMap<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.row_range.0..self.row_range.1 {
            for col in self.col_range.0..self.col_range.1 {
                f.write_char(if self.holes.contains_key(&(row, col)) {
                    '#'
                } else {
                    '.'
                })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn instruction(input: &str) -> IResult<&str, DigInstruction> {
    tuple((
        alt((
            value(Direction::Up, tag("U")),
            value(Direction::Down, tag("D")),
            value(Direction::Left, tag("L")),
            value(Direction::Right, tag("R")),
        ))
        .terminated(tag(" ")),
        nom::character::complete::i64.terminated(tag(" ")),
        delimited(
            tag("(#"),
            take_while1(|c: char| c.is_alphanumeric()),
            tag(")"),
        ),
    ))
    .map(|(direction, distance, color)| DigInstruction {
        direction,
        distance,
        color,
    })
    .parse(input)
}

fn parse_input(input: &str) -> Vec<DigInstruction> {
    let (r, result) = separated_list1(line_ending, instruction)
        .parse(input)
        .expect("valid input");
    assert_eq!(r, "");
    result
}

type Point = (i64, i64);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct Line {
    tl: Point,
    br: Point,
}

impl Line {
    fn is_valid(&self) -> bool {
        (self.is_horizontal() || self.is_vertical())
            && (self.br.0 >= self.tl.0)
            && self.br.1 >= self.tl.1
    }

    fn validate(&self) {
        if !self.is_valid() {
            panic!("Invalid line: {:?}", self);
        }
    }

    fn horizontal(tl: Point, d: usize) -> Self {
        Self {
            tl,
            br: (tl.0, tl.1 + d as i64),
        }
    }

    fn vertical(tl: Point, d: usize) -> Self {
        Self {
            tl,
            br: (tl.0 + d as i64, tl.1),
        }
    }

    fn is_horizontal(&self) -> bool {
        self.tl.0 == self.br.0
    }

    fn is_vertical(&self) -> bool {
        self.tl.1 == self.br.1
    }

    fn contains(&self, p: (i64, i64)) -> bool {
        (self.tl.0..=self.br.0).contains(&p.0) && (self.tl.1..=self.br.1).contains(&p.1)
    }

    fn start(&self) -> (i64, i64) {
        self.tl
    }

    fn end(&self) -> (i64, i64) {
        self.br
    }
}

// A dig map that contains a list of lines (since the lines may be huge)
#[derive(Debug, Default)]
struct DigMap2 {
    lines: HashSet<Line>,
    points: Vec<(i64, i64)>,
}

impl DigMap2 {
    /// NOTE: NOT ok for large maps.
    fn display(&self) -> String {
        let mut rl = 0;
        let mut rh = 0;
        let mut cl = 0;
        let mut ch = 0;
        for line in self.lines.iter() {
            let (s, e) = (line.start(), line.end());

            if rl > s.0 {
                rl = s.0;
            }
            if cl > s.1 {
                cl = s.1;
            }
            if rh < e.0 {
                rh = e.0;
            }
            if ch < e.1 {
                ch = e.1;
            }
        }

        (rl..=rh)
            .map(|r| {
                (cl..=ch)
                    .map(|c| {
                        if self.on_some_line((r, c)) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
                    + "\n"
            })
            .collect()
    }

    fn on_some_line(&self, p: (i64, i64)) -> bool {
        for l in self.lines.iter() {
            if l.contains(p) {
                return true;
            }
        }
        false
    }

    fn perform_instructions(&mut self, instructions: &[DigInstruction]) {
        let mut worker_pos = (0, 0);
        self.points.push(worker_pos);
        for instruction in instructions {
            match instruction.direction {
                Direction::Up => {
                    worker_pos.0 -= instruction.distance;
                    self.lines.insert(Line::vertical(
                        worker_pos,
                        instruction.distance as usize,
                    ));
                }
                Direction::Down => {
                    self.lines.insert(Line::vertical(
                        worker_pos,
                        instruction.distance as usize,
                    ));
                    worker_pos.0 += instruction.distance;
                }
                Direction::Left => {
                    worker_pos.1 -= instruction.distance;
                    self.lines.insert(Line::horizontal(
                        worker_pos,
                        instruction.distance as usize,
                    ));
                }
                Direction::Right => {
                    self.lines.insert(Line::horizontal(
                        worker_pos,
                        instruction.distance as usize,
                    ));
                    worker_pos.1 += instruction.distance;
                }
            }
            if worker_pos != (0, 0) {
                self.points.push(worker_pos);
            }
        }
        for l in self.lines.iter() {
            l.validate();
        }
    }

    fn compute_points(&self) -> Vec<Point> {
        // FIXME: compute the points that we are to
        // use to display
        let top_left = self
            .lines
            .iter()
            .map(|l| l.start())
            .min_by(|a, b| {
                if a.0 != b.0 {
                    a.0.cmp(&b.0)
                } else {
                    a.1.cmp(&b.1)
                }
            })
            .expect("has lines");

        let mut result = Vec::new();
        // once top-let is computed, the are MUST be lower-right.
        result.push(top_left);
        let actual = self.points.clone();
        let idx = actual
            .iter()
            .position(|p| p == &top_left)
            .expect("top_left in iterator");
        let (l, r) = actual.split_at(idx);

        let mut reordered = r.iter().collect::<Vec<_>>();
        for x in l {
            reordered.push(x);
        }
        reordered.push(reordered.first().expect("not empty"));
        trace!("Reordered: {:?}", &reordered);

        let mut line_data = Vec::new();

        for (f, t) in reordered.iter().zip(reordered.iter().skip(1)) {
            let d = if f.0 == t.0 {
                // horizontal
                if f.1 < t.1 {
                    Direction::Right
                } else {
                    Direction::Left
                }
            } else {
                assert_eq!(f.1, t.1);
                if f.0 < t.0 {
                    Direction::Down
                } else {
                    Direction::Up
                }
            };
            line_data.push((f, t, d));
        }

        // inside is ALWAYS to the right of the current line. Always insert the end
        for ((_, (r, c), dc), (_, _, dn)) in line_data.iter().zip(line_data.iter().skip(1)) {
            // figure out where we end
            match (dc, dn) {
                (Direction::Up, Direction::Left) => result.push((*r + 1, *c)),
                (Direction::Up, Direction::Right) => result.push((*r, *c)),
                (Direction::Down, Direction::Left) => result.push((*r + 1, *c + 1)),
                (Direction::Down, Direction::Right) => result.push((*r, *c + 1)),
                (Direction::Left, Direction::Up) => result.push((*r + 1, *c)),
                (Direction::Left, Direction::Down) => result.push((*r + 1, *c + 1)),
                (Direction::Right, Direction::Up) => result.push((*r, *c)),
                (Direction::Right, Direction::Down) => result.push((*r, *c + 1)),
                _ => panic!("Invalid combination"),
            }
        }

        trace!("POINTS: {:?}", result);

        // at this point start moving and decide where the area resides
        result
    }

    fn area_from_points(&self) -> u64 {
        let points = self.compute_points();

        let d = points
            .iter()
            .zip(points.iter().skip(1).chain(&[points[0]]))
            .inspect(|d| {
                trace!("2P: {:?}", d);
            })
            .map(|(a, b)| (a.1 * b.0 - b.1 * a.0))
            .inspect(|d| {
                trace!(" ==> {:?}", d);
            })
            .sum::<i64>();

        trace!("FINAL SUM: {}", d);

        assert!(d % 2 == 0);
        (d / 2) as u64
    }
}

#[instrument(skip_all)]
pub fn part1(input: &str) -> usize {
    let mut map = DigMap::new();
    map.perform_instructions(&parse_input(input));
    //info!("DigMap:\n{}", &map);
    map.flood_fill_inside();

    //info!("After dig:\n{}", &map);
    map.dug_out_depth()
}

#[instrument(skip_all)]
pub fn part1_b(input: &str) -> u64 {
    let mut map = DigMap2::default();
    map.perform_instructions(&parse_input(input));
    info!("DigMap:\n{}", map.display());
    info!("DigMapP:\n{:?}", map.points);

    map.area_from_points()
}

pub fn part2(input: &str) -> usize {
    let mut adjusted = Vec::new();
    for i in parse_input(input) {
        adjusted.push(i.color_to_distance());
    }

    let mut map = DigMap2::default();
    map.perform_instructions(&adjusted);
    map.area_from_points() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../example.txt")), 62);
    }

    #[test_log::test]
    fn test_part1_b() {
        assert_eq!(part1_b(include_str!("../example.txt")), 62);
    }

    #[test_log::test]
    fn test_trace() {
        assert_eq!(
            part1_b(
                "
R 2 (#123123)
D 2 (#123123)
R 2 (#123123)
U 2 (#123123)
R 3 (#123123)
D 4 (#123123)
L 3 (#123123)
D 4 (#123123)
L 2 (#123123)
U 2 (#123123)
L 2 (#123123)
U 6 (#123123)
        "
                .trim()
            ),
            54
        );
    }

    #[test_log::test]
    fn test_square1() {
        assert_eq!(
            part1_b(
                "
R 2 (#123123)
D 2 (#123123)
L 2 (#123123)
U 2 (#123123)
        "
                .trim()
            ),
            9
        );
    }

    #[test_log::test]
    fn test_simple1() {
        assert_eq!(
            part1_b(
                "
R 4 (#123123)
D 4 (#123123)
L 2 (#123123)
U 2 (#123123)
L 2 (#123123)
U 2 (#123123)
        "
                .trim()
            ),
            21
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../example.txt")), 952408144115);
    }
}
