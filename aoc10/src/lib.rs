use nom::{
    branch::alt, bytes::complete::tag, character::complete::line_ending, combinator::value,
    multi::{many1, separated_list1}, IResult, Parser,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum MapPoint {
    Ground,
    Pipe(Direction, Direction),
    Start,
}

struct Line {
    points: Vec<MapPoint>,
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    many1(alt((
        value(MapPoint::Ground, tag(".")),
        value(MapPoint::Pipe(Direction::Left, Direction::Right), tag(".")),
        value(MapPoint::Pipe(Direction::Left, Direction::Right), tag("-")),
        value(MapPoint::Pipe(Direction::Up, Direction::Down), tag("|")),
        value(MapPoint::Pipe(Direction::Up, Direction::Right), tag("L")),
        value(MapPoint::Pipe(Direction::Up, Direction::Left), tag("J")),
        value(MapPoint::Pipe(Direction::Down, Direction::Right), tag("7")),
        value(MapPoint::Pipe(Direction::Down, Direction::Left), tag("F")),
        value(MapPoint::Start, tag("S")),
    )))
    .map(|points| Line { points })
    .parse(input)
}

struct Map {
    lines: Vec<Line>,
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    separated_list1(line_ending, parse_line)
        .map(|lines| Map { lines })
        .parse(input)
}

pub fn part1(input: &str) -> u32 {
    let (r, map) = parse_map(input).expect("valid input");
    assert_eq!(r, "");
   0
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../example1.txt")), 4);
        assert_eq!(part1(include_str!("../example2.txt")), 8);
    }
}
