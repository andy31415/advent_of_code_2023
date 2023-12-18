use std::{
    collections::{BTreeMap, HashSet},
    fmt::{Display, Write},
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
use tracing::{info, instrument};

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
        return self.holes.contains_key(&p)
    }
    
    fn find_inside(&self) -> (i64, i64) {
       for row in self.row_range.0..self.row_range.1 {
           for col in self.col_range.0..self.col_range.1 {
               let p = (row, col);

            if !self.hole_at(Direction::Left + p)
                && self.hole_at(p)
                && !self.hole_at(Direction::Right + p) {
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
            
            for d in [Direction::Left, Direction::Right, Direction::Up, Direction::Down] {
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

#[instrument(skip_all)]
pub fn part1(input: &str) -> usize {
    let mut map = DigMap::new();
    map.perform_instructions(&parse_input(input));
    info!("DigMap:\n{}", &map);
    map.flood_fill_inside();

    info!("After dig:\n{}", &map);
    map.dug_out_depth()
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
    fn test_trace() {
        assert_eq!(part1("
R 2 (#123123)
D 2 (#123123)
R 2 (#123123)
U 2 (#123123)
R 3 (#123123)
D 4 (#123123)
L 7 (#123123)
U 4 (#123123)
        ".trim()), 38);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../example.txt")), 0);
    }
}
