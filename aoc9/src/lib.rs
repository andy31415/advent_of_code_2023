use itertools::Itertools;
use nom::{
    character::complete::{self, newline, space1},
    multi::separated_list1,
    IResult, Parser,
};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Sequence {
    values: Vec<i64>,
}

fn parse_sequence(input: &str) -> IResult<&str, Sequence> {
    separated_list1(space1, complete::i64)
        .map(|values| Sequence { values })
        .parse(input)
}

impl Sequence {
    pub fn towers(&self) -> Vec<Vec<i64>> {
        let mut towers = Vec::new();

        let mut values = self.values.clone();
        while !values.iter().all(|v| *v == 0) {
            towers.push(values.clone());
            values = values.iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect();
        }
        towers
    }

    pub fn next_tower_sum(&self) -> i64 {
        // Computes the next value in a tower...
        self.towers()
            .iter()
            .rev()
            .fold(0, |acc, x| acc + x.last().expect("non-empty"))
    }

    pub fn previous_tower_sum(&self) -> i64 {
        // Computes the next value in a tower...
        self.towers()
            .iter()
            .rev()
            .fold(0, |acc, x| x.first().expect("non-empty") - acc)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Input {
    sequences: Vec<Sequence>,
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    separated_list1(newline, parse_sequence)
        .map(|sequences| Input { sequences })
        .parse(input)
}

pub fn part1(input: &str) -> i64 {
    let (rest, input) = parse_input(input).expect("Valid input");
    assert_eq!(rest, "");

    input.sequences.iter().map(|s| s.next_tower_sum()).sum()
}

pub fn part2(input: &str) -> i64 {
    let (rest, input) = parse_input(input).expect("Valid input");
    assert_eq!(rest, "");

    input.sequences.iter().map(|s| s.previous_tower_sum()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1("0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45"),
            114
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45"), 2);
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45")
                .expect("valid")
                .1,
            Input {
                sequences: {
                    vec![
                        Sequence {
                            values: vec![0, 3, 6, 9, 12, 15],
                        },
                        Sequence {
                            values: vec![1, 3, 6, 10, 15, 21],
                        },
                        Sequence {
                            values: vec![10, 13, 16, 21, 30, 45],
                        },
                    ]
                }
            }
        )
    }

    #[test]
    fn test_parse_sequence() {
        assert_eq!(
            parse_sequence("0 3 6 9 12 15").expect("valid").1,
            Sequence {
                values: vec![0, 3, 6, 9, 12, 15]
            }
        );
    }
}
