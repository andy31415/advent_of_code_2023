use glam::IVec3;
use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult, Parser,
};
use nom_supreme::ParserExt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Brick {
    start: IVec3,
    end: IVec3,
}

fn vec3d(s: &str) -> IResult<&str, IVec3> {
    tuple((
        nom::character::complete::i32.terminated(tag(",")),
        nom::character::complete::i32.terminated(tag(",")),
        nom::character::complete::i32,
    ))
    .map(|(x, y, z)| IVec3::new(x, y, z))
    .parse(s)
}

fn line(s: &str) -> IResult<&str, (IVec3, IVec3)> {
    separated_pair(vec3d, tag("~"), vec3d).parse(s)
}

fn parse_input(s: &str) -> IResult<&str, Vec<Brick>> {
    separated_list1(line_ending, line.map(|(start, end)| Brick { start, end })).parse(s)
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);
    dbg!(input);
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
