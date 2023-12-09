use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, none_of},
    combinator::{recognize, value},
    multi::{many1, many_m_n},
    sequence::tuple,
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
struct Location<'a> {
    name: &'a str,
    ghost_start: bool,
    ghost_end: bool,
}

impl<'a> Location<'a> {
    fn new(name: &'a str) -> Self {
        let ghost_start = name.chars().last() == Some('A');
        let ghost_end = name.chars().last() == Some('Z');
        Self {
            name,
            ghost_start,
            ghost_end,
        }
    }
    fn is_ghost_start(&self) -> bool {
        self.ghost_start
    }

    fn is_ghost_end(&self) -> bool {
        self.ghost_end
    }
}

fn parse_location(input: &str) -> IResult<&str, Location> {
    recognize(many_m_n(3, 3, none_of("=(), \n")))
        .map(|s| Location::new(s))
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

    return Ok((span, result));
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

impl<'a> DirectionIter<'a> {
    pub fn pos(&self) -> usize {
        self.pos
    }
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

/// A ghost:
///   - Is always on a "stop"
///   - Has a position in time
///   - can always move to the next stop (generally fast amortized time)
#[derive(Debug, PartialEq)]
struct Ghost<'a> {
    time: usize,                                             // current position in time
    position: &'a Location<'a>,                                  // a STOP position in time
    next_stop: HashMap<FillKey<'a>, (usize, &'a Location<'a>)>, // how many steps to move to the next stop
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct FillKey<'a>(usize, &'a Location<'a>);

impl<'a> Ghost<'a> {
    fn new(start: &'a Location<'a>, map: &'a Map<'a>) -> Ghost<'a> {
        // figure out the path of this ghost completely
        assert!(start.is_ghost_start());
        let mut position = start;
        let mut time = 0;

        let mut moves = map.directions.iter();

        while !position.is_ghost_end() {
            position = match moves.next().expect("Moves never end") {
                Direction::Left => &map.map.get(&position).unwrap().0,
                Direction::Right => &map.map.get(&position).unwrap().1,
            };
            time = time + 1;
        }

        // we have a start position. Now figure out all ends
        let position = position;
        let mut next_stop = HashMap::new();

        let mut fill = position;
        let mut fill_pos = FillKey(moves.pos, fill);
        while !next_stop.contains_key(&fill_pos) {
            // given the current pos, find out how many steps left
            let mut steps = 0 as usize;
            loop {
                steps += 1;
                fill = match moves.next().expect("Moves never end") {
                    Direction::Left => &map.map.get(fill).unwrap().0,
                    Direction::Right => &map.map.get(fill).unwrap().1,
                };
                eprintln!("{:?}", fill);

                if fill.is_ghost_end() {
                    break;
                }
            }
            eprintln!("{:?} -> {:?}", fill_pos, (steps, fill));
            next_stop.insert(fill_pos, (steps, fill));
            // figure out from where we have to move
            fill_pos = FillKey(moves.pos, fill);
        }

        Ghost {
            time,
            position,
            next_stop,
        }
    }
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
    let target = Location::new("ZZZ");
    let mut position = &Location::new("AAA");

    for (i, d) in map.directions.iter().enumerate() {
        position = match d {
            Direction::Left => &map.map.get(&position).expect("valid").0,
            Direction::Right => &map.map.get(&position).expect("valid").1,
        };

        if *position == target {
            return i + 1;
        }
    }

    panic!("should never finish")
}

pub fn part2_steps(input: &str) -> usize {
    let map: Map = parse_input(input).expect("valid input").1.into();
    
    let mut ghost_positions = map
        .map
        .keys()
        .filter(|k| k.is_ghost_start())
        .collect::<HashSet<_>>();
    
    let mut ghosts = ghost_positions.iter().map(|p| 
                                                Ghost::new(&p, &map)
    ).collect::<Vec<_>>();

    eprintln!("GHOSTS: {:#?}", ghosts);

    /*
    for (i, d) in map.directions.iter().enumerate() {
        // move all ghost positions
        ghost_positions = ghost_positions
            .iter()
            .map(|position| match d {
                Direction::Left => &map.map.get(&position).expect("valid").0,
                Direction::Right => &map.map.get(&position).expect("valid").1,
            })
            .collect();

        if ghost_positions.iter().all(|p| p.is_ghost_end()) {
            return i + 1;
        }
    }*/
    

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
    fn test_part2() {
        assert_eq!(part2_steps(include_str!("../example2.txt")), 6);
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
                        key: Location::new("AAA"),
                        left: Location::new("BBB"),
                        right: Location::new("CCC")
                    },
                    LocationMap {
                        key: Location::new("BBB"),
                        left: Location::new("DDD"),
                        right: Location::new("EEE")
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
                key: Location::new("AAA"),
                left: Location::new("BBB"),
                right: Location::new("CCC")
            }
        );
    }
}
