use std::collections::{HashMap};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{multispace1, multispace0, none_of},
    },
    combinator::{recognize, value},
    multi::{many1, many_m_n},
    sequence::{tuple},
    IResult, Parser,
};
use nom_supreme::ParserExt;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Direction {
    Left,
    Right,
}

fn parse_direction_list(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(alt((
        value(Direction::Left, tag("L")),
        value(Direction::Right, tag("R")),
    )))
    .parse(input)
}

// a location, generally 3-letter location
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Location<'a>(&'a str);

fn parse_location(input: &str) -> IResult<&str, Location> {
    recognize(many_m_n(3, 3, none_of("=(), \n")))
        .map(|s| Location(s))
        .parse(input)
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct LocationMap<'a> {
    key: Location<'a>,
    left: Location<'a>,
    right: Location<'a>,
}

fn parse_location_map(input: &str) -> IResult<&str, LocationMap> {
    tuple((
        parse_location,
        parse_location.preceded_by(tag(" = (")),
        parse_location.preceded_by(tag(", ")).terminated(tag(")")),
    ))
    .map(|(key, left, right)| LocationMap { key, left, right })
    .parse(input)
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
struct InputData<'a> {
    directions: Vec<Direction>,
    map_list: Vec<LocationMap<'a>>,
}

fn parse_input(input: &str) -> IResult<&str, InputData> {
    let (span, result) = tuple((
        parse_direction_list.terminated(multispace1),
        many1(parse_location_map.terminated(multispace0)),
    ))
    .map(|(directions, map_list)| InputData {
        directions,
        map_list,
    })
    .parse(input)?;
    
    assert_eq!(span, "");

    return Ok((span, result))
}

struct DirectionLoop {
    steps: Vec<Direction>,
}

impl DirectionLoop {
    pub fn iter(&self) -> DirectionIter {
        DirectionIter {
            steps: &self.steps,
            pos: 0,
        }
    }
}

struct DirectionIter<'a> {
    steps: &'a Vec<Direction>,
    pos: usize,
}

impl<'a> Iterator for DirectionIter<'a> {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.steps.get(self.pos).expect("Non-empty iterator");

        self.pos += 1;
        if self.pos >= self.steps.len() {
            self.pos = 0;
        }

        return Some(*result);
    }
}

struct Map<'a> {
    directions: DirectionLoop,
    map: HashMap<Location<'a>, (Location<'a>, Location<'a>)>,
}

impl<'a> Into<Map<'a>> for InputData<'a> {
    fn into(self) -> Map<'a> {
        let mut map = HashMap::new();
        for k in self.map_list {
            map.insert(k.key, (k.left, k.right));
        }

        Map {
            directions: DirectionLoop {
                steps: self.directions,
            },
            map,
        }
    }
}

pub fn part1_steps(input: &str) -> usize {
    let map: Map = parse_input(input).expect("valid input").1.into();
    let target = Location("ZZZ");
    let mut position = &Location("AAA");

    for (i, d) in map.directions.iter().enumerate() {
        position = match d {
            Direction::Left => &map.map.get(&position).expect("valid").0,
            Direction::Right => &map.map.get(&position).expect("valid").1,
        };

        if *position == target {
            return i+1;
        }
    }
    
    panic!("should never finish")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1_steps(include_str!("../example.txt")), 6);
    }

    #[test]
    fn test_direction_loop_iterate() {
        let d = DirectionLoop {
            steps: vec![Direction::Left, Direction::Left, Direction::Right],
        };

        assert_eq!(
            d.iter().take(10).collect::<Vec<_>>(),
            vec![
                Direction::Left,
                Direction::Left,
                Direction::Right,
                Direction::Left,
                Direction::Left,
                Direction::Right,
                Direction::Left,
                Direction::Left,
                Direction::Right,
                Direction::Left,
            ]
        );
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("RLR\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)")
                .expect("ok")
                .1,
            InputData {
                directions: vec![Direction::Right, Direction::Left, Direction::Right,],
                map_list: vec![
                    LocationMap {
                        key: Location("AAA"),
                        left: Location("BBB"),
                        right: Location("CCC")
                    },
                    LocationMap {
                        key: Location("BBB"),
                        left: Location("DDD"),
                        right: Location("EEE")
                    }
                ]
            }
        );
    }

    #[test]
    fn test_parse_directions() {
        assert_eq!(
            parse_direction_list("RLLRRL").expect("ok").1,
            vec![
                Direction::Right,
                Direction::Left,
                Direction::Left,
                Direction::Right,
                Direction::Right,
                Direction::Left,
            ]
        );
    }

    #[test]
    fn test_parse_location_map() {
        assert_eq!(
            parse_location_map("AAA = (BBB, CCC)").expect("ok").1,
            LocationMap {
                key: Location("AAA"),
                left: Location("BBB"),
                right: Location("CCC")
            }
        );
    }
}
