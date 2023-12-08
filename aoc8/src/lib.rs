use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{multispace1, one_of},
        is_alphabetic,
    },
    combinator::{recognize, value},
    multi::{many1, many_m_n, separated_list1},
    sequence::{pair, tuple},
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
    recognize(many_m_n(3, 3, one_of("ABCDEFGHIJKLMNOPQRST")))
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
    tuple((
        parse_direction_list.terminated(multispace1),
        separated_list1(multispace1, parse_location_map),
    ))
    .map(|(directions, map_list)| InputData {
        directions,
        map_list,
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("RLR\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)")
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
