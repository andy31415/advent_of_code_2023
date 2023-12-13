use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, space1, multispace1},
    combinator::value,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};
use nom_supreme::ParserExt;

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Copy, Clone)]
pub enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

fn spring_state(input: &str) -> IResult<&str, SpringState> {
    alt((
        value(SpringState::Operational, tag(".")),
        value(SpringState::Damaged, tag("#")),
        value(SpringState::Unknown, tag("?")),
    ))
    .parse(input)
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct SpringLine {
    states: Vec<SpringState>,
    runs: Vec<u32>,
}

fn spring_line(input: &str) -> IResult<&str, SpringLine> {
    separated_pair(
        many1(spring_state),
        space1,
        separated_list1(tag(","), nom::character::complete::u32),
    )
    .map(|(states, runs)| SpringLine { states, runs })
    .parse(input)
}

struct Input {
    lines: Vec<SpringLine>,
}

fn parse_input(i: &str) -> IResult<&str, Input> {
    separated_list1(multispace1, spring_line)
        .map(|lines| Input { lines })
        .parse(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let (r, d) = parse_input(include_str!("../example.txt")).expect("valid");
        assert_eq!(r, "");
        assert_eq!(d.lines.len(), 6);
    }

    #[test]
    fn test_spring_line_parse() {
        assert_eq!(
            spring_line(".#.###.#.###### 1,3,1,6").expect("valid").1,
            SpringLine {
                states: vec![
                    SpringState::Operational,
                    SpringState::Damaged,
                    SpringState::Operational,
                    SpringState::Damaged,
                    SpringState::Damaged,
                    SpringState::Damaged,
                    SpringState::Operational,
                    SpringState::Damaged,
                    SpringState::Operational,
                    SpringState::Damaged,
                    SpringState::Damaged,
                    SpringState::Damaged,
                    SpringState::Damaged,
                    SpringState::Damaged,
                    SpringState::Damaged,
                ],
                runs: vec![1, 3, 1, 6],
            }
        );

        assert_eq!(
            spring_line("???.### 1,1,3").expect("valid").1,
            SpringLine {
                states: vec![
                    SpringState::Unknown,
                    SpringState::Unknown,
                    SpringState::Unknown,
                    SpringState::Operational,
                    SpringState::Damaged,
                    SpringState::Damaged,
                    SpringState::Damaged,
                ],
                runs: vec![1, 1, 3],
            }
        );
    }
}
