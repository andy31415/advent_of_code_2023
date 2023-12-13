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

fn consume_damage(input: &[SpringState], amount: usize) -> Option<&[SpringState]> {
    // try to consume these many damaged states from the input
    // return None on failure
    if input.len() < amount {
        return None;
    }

    let (left, right) = input.split_at(amount);

    if left.iter().any(|s| *s == SpringState::Operational) {
        // insufficient run
        return None;
    }

    match right {
        [] => Some(&[]), // run ends at end of array
        [first, rest @ ..] => {
            if *first == SpringState::Damaged {
                None // Not an end of the run
            } else {
                Some(rest)
            }
        }
    }
}

fn match_possibilities(states: &[SpringState], runs: &[u32], depth: usize) -> u32 {
    trace!(
        "{:indent$} IN {:?}/{:?}",
        "",
        states,
        runs,
        indent = depth * 2
    );
    match runs {
        [] => {
            let total = if states.iter().any(|s| *s == SpringState::Damaged) {
                0
            } else {
                1
            };
            trace!(
                "{:indent$} OUT {:?}/{:?} => {} (final)",
                "",
                states,
                runs,
                total,
                indent = depth * 2
            );
            total
        }
        [first, tail_runs @ ..] => {
            let mut total = 0;

            // try to consume damage now
            if let Some(tail_states) = consume_damage(states, *first as usize) {
                trace!("{:indent$} CONSUMED {}", "", *first, indent = depth * 2);
                total += match_possibilities(tail_states, tail_runs, depth + 1)
            }

            // if current state is not damage, try to also recurse without consuming damage yet
            match states {
                [] => (),                        // non-empty runs, no match
                [SpringState::Damaged, ..] => (), // damage, must be in a run
                [_, tail_states @ ..] => {
                    trace!("{:indent$} SKIP operational", "", indent = depth * 2);
                    total += match_possibilities(tail_states, runs, depth + 1);
                },
            }

            total
        }
    }
}

impl SpringLine {
    fn possibilities(&self) -> u32 {
        match_possibilities(self.states.as_slice(), self.runs.as_slice(), 0)
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

    fn spring_line_items(s: &str) -> Vec<SpringState> {
        let (s, r) = many1(spring_state)(s).expect("valid input");
        assert_eq!(s, "");

        r
    }

    #[test]
    fn test_consume_damage() {
        assert_eq!(
            consume_damage(spring_line_items("???.###").as_slice(), 3),
            Some(spring_line_items("###").as_slice())
        );
        assert_eq!(
            consume_damage(spring_line_items("???.###").as_slice(), 1),
            Some(spring_line_items("?.###").as_slice())
        );
        assert_eq!(
            consume_damage(spring_line_items("???.###").as_slice(), 2),
            Some(spring_line_items(".###").as_slice())
        );
        assert_eq!(
            consume_damage(spring_line_items("?#?.###").as_slice(), 2),
            Some(spring_line_items(".###").as_slice())
        );
        assert_eq!(
            consume_damage(spring_line_items(".??.###").as_slice(), 1),
            None
        );
        assert_eq!(
            consume_damage(spring_line_items("#.?.###").as_slice(), 2),
            None
        );
        assert_eq!(
            consume_damage(spring_line_items("###.###").as_slice(), 3),
            Some(spring_line_items("###").as_slice())
        );
    }

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
            spring_line("?###???????? 3,2,1")
                .expect("valid")
                .1
                .possibilities(),
            10
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
        assert_eq!(spring_line("???? 1").expect("valid").1.possibilities(), 4);
        assert_eq!(spring_line("??? 1").expect("valid").1.possibilities(), 3);
        assert_eq!(spring_line("?? 1").expect("valid").1.possibilities(), 2);
        assert_eq!(spring_line("? 1").expect("valid").1.possibilities(), 1);
        assert_eq!(
            spring_line("??????? 2,1").expect("valid").1.possibilities(),
            10
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
