use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, space1},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult, Parser, combinator::recognize,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Race {
    time: u32,
    record: u32,
}

impl Race {
    pub fn trave_distance(&self, press: u32) -> u32 {
        return (self.time - press) * press;
    }

    pub fn win_counts(&self) -> usize {
        (1..self.time)
            .map(|p| self.trave_distance(p))
            .filter(|n| *n > self.record)
            .count()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InputData {
    pub races: Vec<Race>,
}

pub fn parse_input_kernig(input: &str) -> IResult<&str, InputData> {
    tuple((
        delimited(
            tuple((tag("Time:"), space1)),
            separated_list1(space1, recognize(nom::character::complete::u32)),
            multispace1,
        ).map(|items| {
            items.join("").parse()
        }),
        delimited(
            tuple((tag::<&str, _, _>("Distance:"), space1)),
            separated_list1(space1, recognize(nom::character::complete::u32)),
            multispace0,
        ).map(|items|{
            items.join("").parse()
        }),
    ))
    .map(|(time, distance)| InputData {
        races: time
            .iter()
            .zip(distance)
            .map(|(time, record)| Race {
                time: *time,
                record,
            })
            .collect(),
    })
    .parse(input)
}

pub fn parse_input(input: &str) -> IResult<&str, InputData> {
    tuple((
        delimited(
            tuple((tag("Time:"), space1)),
            separated_list1(space1, nom::character::complete::u32),
            multispace1,
        ),
        delimited(
            tuple((tag::<&str, _, _>("Distance:"), space1)),
            separated_list1(space1, nom::character::complete::u32),
            multispace0,
        ),
    ))
    .map(|(time, distance)| InputData {
        races: time
            .iter()
            .zip(distance)
            .map(|(time, record)| Race {
                time: *time,
                record,
            })
            .collect(),
    })
    .parse(input)
}

pub fn part_1(input: &str) -> usize {
    let data = parse_input(input).expect("valid input").1;
    data.races.iter().map(|r| r.win_counts()).product()
}

pub fn part_2(input: &str) -> usize {
    let data = parse_input_kernig(input).expect("valid input").1;
    data.races.iter().map(|r| r.win_counts()).product()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part_1(include_str!("../example.txt")), 288);
    }


    #[test]
    fn test_part2() {
        assert_eq!(part_2(include_str!("../example.txt")), 71503);
    }

    #[test]
    fn test_parse_input_kernig() {
        assert_eq!(
            parse_input_kernig(include_str!("../example.txt"))
                .expect("valid input")
                .1,
            InputData {
                races: vec![
                    Race { time: 71530, record: 940200 },
                ]
            });
        }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(include_str!("../example.txt"))
                .expect("valid input")
                .1,
            InputData {
                races: vec![
                    Race { time: 7, record: 9 },
                    Race {
                        time: 15,
                        record: 40
                    },
                    Race {
                        time: 30,
                        record: 200
                    },
                ]
            }
        );
    }
}
