use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::{Display, Write},
    ops::Add,
};

use glam::IVec2;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::{
        complete::{line_ending, one_of},
        is_alphanumeric,
    },
    combinator::value,
    multi::{many1, separated_list1},
    sequence::{delimited, tuple},
    IResult, Parser,
};
use nom_supreme::ParserExt;
use tracing::{info, instrument};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn tuple(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

impl Add<(i32, i32)> for Direction {
    type Output = (i32, i32);

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        let t = self.tuple();
        (rhs.0 + t.0, rhs.1 + t.1)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct DigInstruction<'a> {
    direction: Direction,
    distance: i32, // always positive, but easier math
    color: &'a str,
}

struct DigMap {
    // locations of holes
    holes: BTreeSet<(i32, i32)>,
    row_range: (i32, i32), // upper range is exclusive
    col_range: (i32, i32), // upper range is exclusive

    // digger position
    digger_pos: (i32, i32),
}

impl DigMap {
    fn new() -> Self {
        let mut holes = BTreeSet::new();
        let digger_pos = (0, 0);
        holes.insert(digger_pos);

        Self {
            holes,
            row_range: (0, 1),
            col_range: (0, 1),
            digger_pos,
        }
    }

    fn perform_instructions(&mut self, instructions: &[DigInstruction]) {
        for instruction in instructions {
            for _ in 0..instruction.distance {
                self.digger_pos = instruction.direction + self.digger_pos;
                self.holes.insert(self.digger_pos);

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
}

impl Display for DigMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.row_range.0..self.row_range.1 {
            for col in self.col_range.0..self.col_range.1 {
                f.write_char(if self.holes.contains(&(row, col)) {
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
        nom::character::complete::i32.terminated(tag(" ")),
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

#[instrument(skip_all)]
pub fn part1(input: &str) -> usize {
    let mut map = DigMap::new();
    map.perform_instructions(&parse_input(input));
    
    info!("DigMap:\n{}", &map);
    
    // TODO: implement
    0
}

pub fn part2(input: &str) -> usize {
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../example.txt")), 0);
    }
}