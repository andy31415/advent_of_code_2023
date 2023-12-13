use std::fmt::{Display, Write};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, multispace1, space1},
    combinator::value,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};
use tracing::{debug, info, trace};

#[derive(Ord, PartialOrd, PartialEq, Eq, Copy, Clone)]
pub enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

impl std::fmt::Debug for SpringState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpringState::Operational => f.write_char('.'),
            SpringState::Damaged => f.write_char('#'),
            SpringState::Unknown => f.write_char('?'),
        }
    }
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

fn match_possibilities(
    states: &[SpringState],
    current_run: u32,
    rest_of_runs: &[u32],
    depth: usize,
) -> u32 {
    // current run: what errors we MUST finish
    trace!("{:indent$}IN:   {:?} {:?},{:?}","", states, current_run, rest_of_runs, indent=depth*2);
    if current_run > 0 {
        if current_run == 1 {
            // MUST be a valid state AND have a separator
            return match states {
                [] => 0, // empty is invalid
                [SpringState::Damaged | SpringState::Unknown] => {
                    if rest_of_runs == [] {
                        1
                    } else {
                        0
                    }
                }
                [SpringState::Operational, ..] => 0, // not a valid stop
                [SpringState::Damaged | SpringState::Unknown, SpringState::Operational | SpringState::Unknown, rest @ ..] =>
                {
                    let mut total = match_possibilities(rest, 0, rest_of_runs, depth + 1);

                    if let [first, tail @ ..] = rest_of_runs {
                        total += match_possibilities(rest, *first, tail, depth + 1);
                    }
                    total
                }
                [_, SpringState::Damaged, ..] => 0, // not a valid end of run
            };
        }

        return match states {
            [] => 0,                             // empty,
            [SpringState::Operational, ..] => 0, // not a valid stop
            [_, rest @ ..] => match_possibilities(rest, current_run - 1, rest_of_runs, depth + 1),
        };
    }

    // current run is 0. figure out if we must start a run or not
    return match states {
        [] => {
            if rest_of_runs == [] {
                1
            } else {
                0
            }
        }
        [first, rest @ ..] => {
            // Choice: start a new run or not
            let mut total = 0;
            if *first == SpringState::Operational || *first == SpringState::Unknown {
                total += match_possibilities(rest, current_run, rest_of_runs, depth + 1)
            }
            if *first == SpringState::Damaged || *first == SpringState::Unknown {
                match rest_of_runs {
                    [] => (),
                    [run, tail @ ..] => total += match_possibilities(states, *run, tail, depth + 1),
                }
            }
            total
        }
    };
}

impl SpringLine {
    fn possibilities(&self) -> u32 {
        match self.runs.as_slice() {
            [] => {
                if self.states.iter().any(|s| *s == SpringState::Damaged) {
                    0
                } else {
                    1
                }
            }
            [start, tail @ ..] => match_possibilities(self.states.as_slice(), *start, tail, 0),
        }
    }
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

pub fn part1(i: &str) -> u32 {
    let (r, d) = parse_input(i).expect("valid input");
    assert_eq!(r, "");

    d.lines.iter().map(|l| l.possibilities()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_runs_simple() {
        assert_eq!(
            spring_line("???.### 1,1,3")
                .expect("valid")
                .1
                .possibilities(),
            1
        );
    }

    #[test_log::test]
    fn test_runs_debug_cases() {
        assert_eq!(
            spring_line("???.# 2,1").expect("valid").1.possibilities(),
            2
        );
    }

    #[test]
    fn test_runs_complex() {
        assert_eq!(
            spring_line(".??..??...?##. 1,1,3")
                .expect("valid")
                .1
                .possibilities(),
            4
        );
        assert_eq!(
            spring_line("?#?#?#?#?#?#?#? 1,3,1,6")
                .expect("valid")
                .1
                .possibilities(),
            1
        );
        assert_eq!(
            spring_line("????.#...#... 4,1,1")
                .expect("valid")
                .1
                .possibilities(),
            1
        );
        assert_eq!(
            spring_line("????.######..#####. 1,6,5")
                .expect("valid")
                .1
                .possibilities(),
            4
        );
        assert_eq!(
            spring_line("?###???????? 3,2,1")
                .expect("valid")
                .1
                .possibilities(),
            10
        );
    }

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
