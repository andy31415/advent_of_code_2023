use std::{
    cmp::min,
    fmt::{Display, Write},
};

use ndarray::Array2;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::value,
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult, Parser,
};
use tracing::info;

#[derive(Debug, PartialEq)]
pub struct Puzzle {
    pub data: Array2<bool>,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Mirror {
    AfterRow(usize),
    AfterCol(usize),
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in self.data.rows() {
            for c in r {
                match c {
                    true => f.write_char('#')?,
                    false => f.write_char('.')?,
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Puzzle {
    fn symmetric_after_col(&self, col: usize) -> bool {
        let cols = self.data.ncols();
        for delta in 0..=min(col, cols - col - 2) {
            if self.data.column(col - delta) != self.data.column(col + delta + 1) {
                return false;
            }
        }
        true
    }

    fn symmetric_after_row(&self, row: usize) -> bool {
        let rows = self.data.nrows();
        for delta in 0..=min(row, rows - row - 2) {
            if self.data.row(row - delta) != self.data.row(row + delta + 1) {
                return false;
            }
        }
        true
    }

    fn find_symmetry(&self) -> Option<Mirror> {
        info!("CHECKING:\n{}\n\n", self);
        // find which row or column is symmetric
        for col in 0..(self.data.ncols() - 1) {
            if self.symmetric_after_col(col) {
                return Some(Mirror::AfterCol(col));
            }
        }

        for row in 0..(self.data.nrows() - 1) {
            if self.symmetric_after_row(row) {
                return Some(Mirror::AfterRow(row));
            }
        }

        None
    }
    
    fn score_symmetry(&self) -> usize {
        match self.find_symmetry() {
            Some(Mirror::AfterCol(n)) => n+1,
            Some(Mirror::AfterRow(n)) => 100*(n+1),
            None => panic!("no symmetry found for {}", self),
        }

    }
}
 
fn puzzle(input: &str) -> IResult<&str, Puzzle> {
    separated_list1(
        line_ending,
        many1(alt((value(false, tag(".")), value(true, tag("#"))))),
    )
    .map(|data| {
        let cols = data.iter().next().expect("Non-empty puzle").len();
        let rows = data.len();

        assert!(data.iter().all(|v| v.len() == cols));

        let raw = data.into_iter().flatten().collect::<Vec<_>>();

        Puzzle {
            data: Array2::from_shape_vec((rows, cols), raw).expect("vector is the right size"),
        }
    })
    .parse(input)
}

#[derive(Debug, PartialEq)]
pub struct Input {
    pub puzzles: Vec<Puzzle>,
}

fn parse_input(input: &str) -> Input {
    let (r, data) = separated_list1(tuple((line_ending, line_ending)), puzzle)
        .map(|puzzles| Input { puzzles })
        .parse(input)
        .expect("Valid input");

    assert_eq!(r, "");

    data
}

pub fn part1(input: &str) -> usize {
    parse_input(input).puzzles.iter().map(|d| d.score_symmetry()).sum()
}

#[cfg(test)]
mod tests {
    use ndarray::array;

    use super::*;

    #[test]
    fn test_parse_input() {
        let p = parse_input(include_str!("../example.txt"));
        assert_eq!(p.puzzles.len(), 2);
        assert!(p.puzzles.iter().all(|p| p.data.dim() == (7, 9)));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../example.txt")), 405);
    }

    #[test_log::test]
    fn test_symmetry() {
        assert_eq!(
            puzzle(
                "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            )
            .expect("valid input")
            .1
            .find_symmetry(),
            Some(Mirror::AfterRow(3))
        );

        assert_eq!(
            puzzle(
                "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
            )
            .expect("valid input")
            .1
            .find_symmetry(),
            Some(Mirror::AfterCol(4))
        );
    }

    #[test]
    fn test_parse_puzzle() {
        assert_eq!(
            puzzle(
                "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
            )
            .expect("valid input")
            .1
            .data,
            array![
                [true, false, true, true, false, false, true, true, false],
                [false, false, true, false, true, true, false, true, false],
                [true, true, false, false, false, false, false, false, true],
                [true, true, false, false, false, false, false, false, true],
                [false, false, true, false, true, true, false, true, false],
                [false, false, true, true, false, false, true, true, false],
                [true, false, true, false, true, true, false, true, false]
            ]
        );
    }
}
