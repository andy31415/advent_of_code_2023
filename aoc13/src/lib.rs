use std::{
    cmp::min,
    fmt::{Display, Write},
};

use ndarray::{Array2, ArrayView1};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::value,
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult, Parser,
};
use tracing::{info, trace};

#[derive(Debug, PartialEq, Clone)]
pub struct Puzzle {
    pub data: Array2<bool>,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Mirror {
    AfterRow(usize),
    AfterCol(usize),
}

impl Mirror {
    pub fn score(&self) -> usize {
        match self {
            Mirror::AfterCol(n) => n + 1,
            Mirror::AfterRow(n) => 100 * (n + 1),
        }
    }
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

fn single_diff(a: ArrayView1<bool>, b: ArrayView1<bool>) -> Option<usize> {
    assert_eq!(a.len(), b.len());

    let mut result: Option<usize> = None;
    for ((idx, va), vb) in a.iter().enumerate().zip(b.iter()) {
        if *va == *vb {
            continue;
        }

        if result.is_none() {
            result = Some(idx);
        } else {
            // two diffs
            return None;
        }
    }

    result
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
struct ColSmudge {
    c1: usize,
    c2: usize,
    row: usize,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
struct RowSmudge {
    col: usize,
    r1: usize,
    r2: usize,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
enum Smudge {
    Col(ColSmudge),
    Row(RowSmudge),
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

    fn flip(&mut self, r: usize, c: usize) {
        let p = self.data.get_mut((r, c)).expect("valid");
        *p = !*p;
    }

    fn fix_smudge(&mut self) -> Option<Mirror> {
        // find two lines that seem to be the same and fixing them
        // results in a different symmetry
        info!("CHECKING SMUDGE IN:\n{}\n\n", self);

        let mut smudge_options = Vec::new();

        for r1 in 0..(self.data.nrows() - 1) {
            for r2 in (r1 + 1)..self.data.nrows() {
                let col = single_diff(self.data.row(r1), self.data.row(r2));
                if let Some(col) = col {
                    trace!("  MAYBE DIFF BY 1 in rows: {},{}", r1, r2);
                    smudge_options.push(Smudge::Row(RowSmudge { r1, r2, col }));
                }
            }
        }

        for c1 in 0..(self.data.ncols() - 1) {
            for c2 in (c1 + 1)..self.data.ncols() {
                let row = single_diff(self.data.column(c1), self.data.column(c2));
                if let Some(row) = row {
                    trace!("  MAYBE DIFF BY 1 in columns: {},{}", c1, c2);
                    smudge_options.push(Smudge::Col(ColSmudge { c1, c2, row }));
                }
            }
        }
        info!("Potential smudges: {:?}", smudge_options);

        for option in smudge_options {
            match option {
                Smudge::Col(c) => {
                    // any row should be ok to flip, pick one
                    self.flip(c.row, c.c1);

                    let symmetry_point = c.c1 + (c.c2 - c.c1) / 2;
                    if self.symmetric_after_col(symmetry_point) {
                        return Some(Mirror::AfterCol(symmetry_point));
                    }

                    // undo the flip if failed
                    self.flip(c.row, c.c1);
                }
                Smudge::Row(r) => {
                    // any col should be ok to flip, pick one
                    self.flip(r.r1, r.col);

                    let symmetry_point = r.r1 + (r.r2 - r.r1) / 2;
                    if self.symmetric_after_row(symmetry_point) {
                        return Some(Mirror::AfterRow(symmetry_point));
                    }
                    
                    // undo the flip if failed
                    self.flip(r.r1, r.col);
                }
            }
        }

        None
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
            Some(m) => m.score(),
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
    parse_input(input)
        .puzzles
        .iter()
        .map(|d| d.score_symmetry())
        .sum()
}

pub fn part2(input: &str) -> usize {
    parse_input(input)
        .puzzles
        .into_iter()
        .map(|d| d.clone().fix_smudge().expect("has smudge").score())
        .sum()
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

    #[test_log::test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../example.txt")), 405);
    }

    #[test_log::test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../example.txt")), 400);
    }

    #[test_log::test]
    fn test_smudge() {
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
            .fix_smudge(),
            Some(Mirror::AfterRow(0))
        );
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
