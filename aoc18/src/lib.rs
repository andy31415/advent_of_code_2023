use glam::IVec2;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::{complete::{one_of, line_ending}, is_alphanumeric},
    combinator::value,
    multi::{many1, separated_list1},
    sequence::{delimited, tuple},
    IResult, Parser,
};
use nom_supreme::ParserExt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn dir(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (-1, 1),
        }
    }

    fn ivec(&self) -> IVec2 {
        let (r, c) = self.dir();
        IVec2 { x: c, y: r }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct DigInstruction<'a> {
    direction: Direction,
    distance: i32, // always positive, but easier math
    color: &'a str,
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
    let (r, result) = separated_list1(line_ending, instruction).parse(input).expect("valid input");
    assert_eq!(r, "");
    result
}

pub fn part1(input: &str) -> usize {
    let instructions = parse_input(input);
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

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../example.txt")), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../example.txt")), 0);
    }
}
