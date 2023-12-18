use std::{
    collections::{BTreeMap, HashSet},
    fmt::{Debug, Display, Write},
    hash::Hash,
    ops::Add,
};

use glam::I64Vec2;
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
        Self {
            direction: self.direction,
            distance: i64::from_str_radix(self.color, 16).expect("valid"),
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
        return self.holes.contains_key(&p);
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

fn rectangle_area(tl: Point, br: Point) -> usize {
    ((br.0 + 1 - tl.0) * (br.1 + 1 - tl.1)) as usize
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct Line {
    tl: Point,
    br: Point,
}

impl Line {
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

    fn distance(&self) -> usize {
        ((self.br.0 - self.tl.0) + (self.br.1 - self.tl.1) + 1) as usize
    }

    fn with_start_moved_to(&self, tl: (i64, i64)) -> Self {
        // MUST keep only horizontal/vertical
        assert!((tl.0 == self.br.0) || (tl.1 == self.br.1));
        Self { tl, br: self.br }
    }

    fn with_end_moved_to(&self, br: (i64, i64)) -> Self {
        // MUST keep only horizontal/vertical
        assert!((br.0 == self.tl.0) || (br.1 == self.tl.1));
        Self { tl: self.tl, br }
    }
}

// A dig map that contains a list of lines (since the lines may be huge)
#[derive(Debug, Default)]
struct DigMap2 {
    lines: HashSet<Line>,
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
                        if self.on_some_line((r, c).into()) {
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
        for instruction in instructions {
            match instruction.direction {
                Direction::Up => {
                    worker_pos.0 -= instruction.distance;
                    self.lines.insert(Line::vertical(
                        worker_pos.into(),
                        instruction.distance as usize,
                    ));
                }
                Direction::Down => {
                    self.lines.insert(Line::vertical(
                        worker_pos.into(),
                        instruction.distance as usize,
                    ));
                    worker_pos.0 += instruction.distance;
                }
                Direction::Left => {
                    worker_pos.1 -= instruction.distance;
                    self.lines.insert(Line::horizontal(
                        worker_pos.into(),
                        instruction.distance as usize,
                    ));
                }
                Direction::Right => {
                    self.lines.insert(Line::horizontal(
                        worker_pos.into(),
                        instruction.distance as usize,
                    ));
                    worker_pos.1 += instruction.distance;
                }
            }
        }
    }

    fn horizontal_with_end_at(&self, p: Point) -> Line {
        *self
            .lines
            .iter()
            .find(|l| l.is_horizontal() && (l.start() == p || l.end() == p))
            .expect("has line with ending")
    }

    fn vertical_with_end_at(&self, p: Point) -> Line {
        trace!("Searching vertical ending at {:?}", p);
        *self
            .lines
            .iter()
            .find(|l| l.is_vertical() && (l.start() == p || l.end() == p))
            .expect("has line with ending")
    }

    fn vertical_with_start_inside(&self, input: Line) -> Line {
        *self
            .lines
            .iter()
            .find(|l| l.is_vertical() && input.contains(l.start()))
            .expect("Find line with start inside")
    }

    fn remove_rectangle(&mut self) -> Option<usize> {
        // Performs in order:
        // - find the top-left most point in the map
        // - find the rectangle to the rigth of it
        // - remove that rectangle (and re-make lines out of it)
        //
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

        let h = self.horizontal_with_end_at(top_left);
        let v_left = self.vertical_with_end_at(top_left);
        let v_right = self.vertical_with_end_at(h.end());

        trace!(
            "BORDERS:\n  H: {:?}\n  V: {:?}\n  V: {:?}",
            h,
            v_left,
            v_right
        );
        assert!(v_left.start() == h.start());
        assert!(v_right.start() == h.end());
        assert!(v_left != v_right);

        // remove the sides of the rectangle
        self.lines.remove(&h);
        self.lines.remove(&v_left);
        self.lines.remove(&v_right);

        let mut size_removed = 0;

        // At this point we have:
        // Horizontal: size of the full cut
        // Vertical: 2 (maybe different) lengths, for which the shortest MUST be cut
        match v_left.distance().cmp(&v_right.distance()) {
            std::cmp::Ordering::Equal => {
                // They are of the same length. we need to merge SEVERAL lines
                todo!();
            }
            std::cmp::Ordering::Less => {
                // left side is shorter
                let h_low = self.horizontal_with_end_at(v_left.end());
                let other_v = self.vertical_with_start_inside(h_low);

                self.lines.remove(&h_low);

                // add them back:
                //   - new top horizontal
                //   - shorter right-side vertical
                let shorter_right = v_right.with_start_moved_to((h_low.end().0, v_right.start().1));
                self.lines.insert(shorter_right);
                size_removed += rectangle_area(top_left, shorter_right.start());

                // Need to move horizontal.
                // End is fixed, need to determine what to do with the start
                let updated_h = h_low
                    .with_end_moved_to((h_low.start().0, v_right.start().1))
                    .with_start_moved_to(other_v.start());

                // since this line remains, keep the distance
                size_removed -= updated_h.distance();

                self.lines.insert(updated_h);
            }
            std::cmp::Ordering::Greater => {
                // right side is shorter
                todo!();
            }
        }

        Some(size_removed)
    }
}

#[instrument(skip_all)]
pub fn part1(input: &str) -> usize {
    let mut map = DigMap::new();
    map.perform_instructions(&parse_input(input));
    info!("DigMap:\n{}", &map);
    map.flood_fill_inside();

    info!("After dig:\n{}", &map);
    map.dug_out_depth()
}

#[instrument(skip_all)]
pub fn part1_b(input: &str) -> usize {
    let mut map = DigMap2::default();
    map.perform_instructions(&parse_input(input));
    info!("DigMap:\n{}", map.display());
    info!("{:?}", map);

    let mut total = 0;

    while let Some(n) = map.remove_rectangle() {
        info!("Updated, {}:\n{}", n, map.display());
        info!("{:?}", map);
        total += n;
    }
    info!("Final, {}:\n{}", total, map.display());
    total
}

pub fn part2(_input: &str) -> usize {
    // TODO: implement
    0
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
    fn test_move_start() {
        assert_eq!(
            Line::horizontal((10, 10).into(), 5).with_start_moved_to((10, 5)),
            Line::horizontal((10, 5).into(), 10)
        );

        assert_eq!(
            Line::horizontal((10, 10).into(), 5).with_start_moved_to((10, 12)),
            Line::horizontal((10, 12).into(), 3)
        );

        assert_eq!(
            Line::vertical((10, 10).into(), 5).with_start_moved_to((5, 10)),
            Line::vertical((5, 10).into(), 10)
        );

        assert_eq!(
            Line::vertical((10, 10).into(), 5).with_start_moved_to((12, 10)),
            Line::vertical((12, 10).into(), 3)
        );
    }

    #[test_log::test]
    fn test_move_end() {
        assert_eq!(
            Line::horizontal((10, 10).into(), 5).with_end_moved_to((10, 20)),
            Line::horizontal((10, 10).into(), 10)
        );

        assert_eq!(
            Line::horizontal((10, 10).into(), 5).with_end_moved_to((10, 12)),
            Line::horizontal((10, 10).into(), 2)
        );

        assert_eq!(
            Line::vertical((10, 10).into(), 5).with_end_moved_to((20, 10)),
            Line::vertical((10, 10).into(), 10)
        );

        assert_eq!(
            Line::vertical((10, 10).into(), 5).with_end_moved_to((12, 10)),
            Line::vertical((10, 10).into(), 2)
        );
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
L 7 (#123123)
U 4 (#123123)
        "
                .trim()
            ),
            38
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../example.txt")), 952408144115);
    }
}
