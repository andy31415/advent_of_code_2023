use std::{
    collections::{BTreeMap},
    fmt::{Write},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace1, space1},
    combinator::value,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};


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
    runs: Vec<u64>,
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

struct MatchMemoization {
    state: BTreeMap<(usize, usize), u64>, // map (len of state, len of runs) -> possibilities
}

impl MatchMemoization {
    fn new() -> Self {
        Self {
            state: BTreeMap::new(),
        }
    }

    fn match_possibilities(&mut self, states: &[SpringState], runs: &[u64]) -> u64 {
        let key = (states.len(), runs.len());
        if let Some(value) = self.state.get(&key) {
            return *value;
        }
        match runs {
            [] => {
                let total = if states.iter().any(|s| *s == SpringState::Damaged) {
                    0
                } else {
                    1
                };
                self.state.insert(key, total);
                total
            }
            [first, tail_runs @ ..] => {
                let mut total = 0;

                // try to consume damage now
                if let Some(tail_states) = consume_damage(states, *first as usize) {
                    total += self.match_possibilities(tail_states, tail_runs)
                }

                // if current state is not damage, try to also recurse without consuming damage yet
                match states {
                    [] => (),                         // non-empty runs, no match
                    [SpringState::Damaged, ..] => (), // damage, must be in a run
                    [_, tail_states @ ..] => {
                        total += self.match_possibilities(tail_states, runs);
                    }
                }

                self.state.insert(key, total);
                total
            }
        }
    }
}

impl SpringLine {
    fn possibilities(&self) -> u64 {
        MatchMemoization::new().match_possibilities(self.states.as_slice(), self.runs.as_slice(), 0)
    }

    fn unfold(self) -> Self {
        let mut states = Vec::new();
        let mut runs = Vec::new();

        for _ in 0..4 {
            states.extend(self.states.iter());
            states.push(SpringState::Unknown);
            runs.extend(self.runs.iter());
        }
        states.extend(self.states.iter());
        runs.extend(self.runs.iter());

        Self { states, runs }
    }
}

fn spring_line(input: &str) -> IResult<&str, SpringLine> {
    separated_pair(
        many1(spring_state),
        space1,
        separated_list1(tag(","), nom::character::complete::u64),
    )
    .map(|(states, runs)| SpringLine { states, runs })
    .parse(input)
}

struct Input {
    lines: Vec<SpringLine>,
}

impl Input {
    fn unfold(self) -> Self {
        // Every line must be multipled by 5
        Self {
            lines: self.lines.into_iter().map(SpringLine::unfold).collect(),
        }
    }
}

fn parse_input(i: &str) -> IResult<&str, Input> {
    separated_list1(multispace1, spring_line)
        .map(|lines| Input { lines })
        .parse(i)
}

pub fn part1(i: &str) -> u64 {
    let (r, d) = parse_input(i).expect("valid input");
    assert_eq!(r, "");

    d.lines.iter().map(|l| l.possibilities()).sum()
}

pub fn part2(i: &str) -> u64 {
    let (r, d) = parse_input(i).expect("valid input");
    assert_eq!(r, "");

    d.unfold().lines.iter().map(|l| l.possibilities()).sum()
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
            spring_line("????.#...#... 4,1,1")
                .expect("valid")
                .1
                .unfold()
                .possibilities(),
            16
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

        assert_eq!(
            spring_line("????.#...#... 4,1,1")
                .expect("valid")
                .1
                .unfold()
                .possibilities(),
            16
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../example.txt")), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../example.txt")), 525152);
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
