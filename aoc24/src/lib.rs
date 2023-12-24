use glam::Vec3;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Hailstone {
    start: Vec3,
    direction: Vec3,
}

mod parse {
    use glam::Vec3;
    use nom::{
        bytes::complete::tag,
        character::complete::{line_ending, space0},
        multi::separated_list1,
        sequence::{separated_pair, tuple},
        IResult, Parser,
    };
    use nom_supreme::ParserExt;

    use crate::Hailstone;

    fn vector(input: &str) -> IResult<&str, Vec3> {
        tuple((
            nom::character::complete::i64,
            nom::character::complete::i64.preceded_by(tuple((space0, tag(","), space0))),
            nom::character::complete::i64.preceded_by(tuple((space0, tag(","), space0))),
        ))
        .map(|(x, y, z)| Vec3::new(x as f32, y as f32, z as f32))
        .parse(input)
    }

    fn hailstone(input: &str) -> IResult<&str, Hailstone> {
        separated_pair(vector, tuple((space0, tag("@"), space0)), vector)
            .map(|(start, direction)| Hailstone { start, direction })
            .parse(input)
    }

    pub fn input(s: &str) -> Vec<Hailstone> {
        let (rest, result) = separated_list1(line_ending, hailstone)
            .parse(s)
            .expect("valid input");
        assert_eq!(rest, "");

        result
    }
}

pub fn part1(input: &str) -> usize {
    let stones = parse::input(input);
    
    eprintln!("Stones: {}", stones.len());
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
        assert_eq!(part1(include_str!("../example.txt")), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../example.txt")), 0);
    }
}
