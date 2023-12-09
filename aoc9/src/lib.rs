use std::iter::from_fn;

use nom::{
    character::complete::{self, newline, space1},
    multi::separated_list1,
    IResult, Parser,
};
use rulinalg::matrix::Matrix;

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
            values = values
                .iter()
                .zip(values.iter().skip(1))
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

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Polynomial {
    coefficients: Vec<f64>, // a0, a1, ... (i.e. lowest to highest)
}

impl Polynomial {
    pub fn evaluate(&self, x: f64) -> f64 {
        let mut mul = 1.0;
        let mut result = 0.0;
        for c in self.coefficients.iter() {
            result += *c * mul;
            mul *= x;
        }
        result
    }
}

impl From<Sequence> for Polynomial {
    fn from(value: Sequence) -> Self {
        let mut values = value.values.clone();
        let mut coefficients = Vec::new();
        while !values.iter().all(|v| *v == 0) {
            coefficients.push(0.0);
            values = values
                .iter()
                .zip(values.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect();
        }

        // create the power matrix
        let n = coefficients.len();
        let m = Matrix::new(
            n,
            n,
            (0..n)
                .flat_map(|m| {
                    let mut v = 1.0;
                    let mut cnt = 0;
                    let powers_of_n = move || {
                        cnt += 1;
                        if cnt > n {
                            return None;
                        }
                        let oldv = v;
                        v *= m as f64;
                        Some(oldv)
                    };
                    from_fn(powers_of_n)
                })
                .collect::<Vec<_>>(),
        );
        dbg!(&m);

        let inverse = m.inverse().expect("must be inversible");
        dbg!(&inverse);
        let c = Matrix::new(
            n,
            1,
            value
                .values
                .iter()
                .take(n)
                .map(|x| *x as f64)
                .collect::<Vec<_>>(),
        );
        dbg!(&c);

        let r = inverse * c;
        dbg!(&r);

        assert_eq!(r.data().len(), n);

        Polynomial {
            coefficients: r.data().clone(),
        }
    }
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
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("0 3 6 9 12 15")]
    #[case("1 3 6 10 15 21")]
    #[case("10 13 16 21 30 45")]
    fn test_evaluate(#[case] data: &str) {
        let values = parse_sequence(data).expect("valid").1.values;
        let p: Polynomial = Sequence {
            values: values.clone(),
        }
        .into();

        dbg!(&p);
        for (x, fx) in values.iter().enumerate() {
            dbg!(x);
            assert_eq!(p.evaluate(x as f64).round(), *fx as f64);
        }
    }

    fn test_polynomial() {
        let p: Polynomial = Sequence {
            values: vec![1, 3, 6, 10, 15, 21],
        }
        .into();
        assert_eq!(
            p,
            Polynomial {
                coefficients: vec![1.0, 1.5, 0.5]
            }
        )
    }

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
