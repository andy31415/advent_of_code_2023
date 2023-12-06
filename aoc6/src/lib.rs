use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, space1},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult, Parser,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Race {
    time: u32,
    record: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct InputData {
    pub races: Vec<Race>,
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

#[cfg(test)]
mod tests {
    use super::*;

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
