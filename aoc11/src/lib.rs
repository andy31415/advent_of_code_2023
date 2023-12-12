use std::collections::BTreeSet;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace1,
    combinator::value,
    multi::{many1, separated_list1},
    Parser,
};
use nom_locate::LocatedSpan;
use tracing::{debug, info};

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub struct Position {
    row: u32,
    col: u32,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Universe {
    galaxies: BTreeSet<Position>,
}

impl Universe {
    fn new<T>(i: T) -> Self
    where
        T: Iterator<Item = Position>,
    {
        Self {
            galaxies: BTreeSet::from_iter(i),
        }
    }

    fn expand(&mut self) {
        // any row or column that has no galaxies gets expanded
        let max_row = self
            .galaxies
            .iter()
            .map(|p| p.row)
            .max()
            .expect("Some data");
        let max_col = self
            .galaxies
            .iter()
            .map(|p| p.col)
            .max()
            .expect("Some data");

        let mut expand_rows = BTreeSet::new();
        for row in 0..=max_row {
            if !self.galaxies.iter().any(|p| p.row == row) {
                expand_rows.insert(row);
            }
        }
        info!("Expanding rows: {:?}", &expand_rows);

        let mut expand_cols = BTreeSet::new();
        for col in 0..=max_col {
            if !self.galaxies.iter().any(|p| p.col == col) {
                expand_cols.insert(col);
            }
        }
        info!("Expanding cols: {:?}", &expand_cols);

        let mut new_galaxies = self.galaxies.clone().into_iter().collect::<Vec<_>>();

        // now move every galaxy as needed
        for row in expand_rows.iter().rev() {
            for g in new_galaxies.iter_mut() {
                if g.row > *row {
                    g.row += 1;
                }
            }
        }
        for col in expand_cols.iter().rev() {
            for g in new_galaxies.iter_mut() {
                if g.col > *col {
                    g.col += 1;
                }
            }
        }

        self.galaxies = BTreeSet::from_iter(new_galaxies.into_iter());
    }

    pub fn all_distances(&self) -> u32 {
        self.galaxies
            .iter()
            .combinations(2)
            .map(|c| {
                assert_eq!(c.len(), 2);
                let p1 = c.get(0).expect("valid");
                let p2 = c.get(1).expect("valid");

                let dr = if p1.row < p2.row {
                    p2.row - p1.row
                } else {
                    p1.row - p2.row
                };

                let dc = if p1.col < p2.col {
                    p2.col - p1.col
                } else {
                    p1.col - p2.col
                };

                debug!("From {:?} to {:?} => {}", p1, p2, (dr + dc));

                dr + dc
            })
            .sum()
    }
}

pub fn universe(span: Span) -> Universe {
    let (rest, universe) = separated_list1(
        multispace1::<Span, nom::error::Error<Span>>,
        many1(alt((
            value(None, tag(".")),
            tag("#").map(|l: Span| {
                Some(Position {
                    row: l.location_line() - 1,
                    col: (l.get_column() - 1) as u32,
                })
            }),
        )))
        .map(|data| {
            data.into_iter()
                .filter_map(|x| x)
                .collect::<Vec<Position>>()
        }),
    )
    .map(|data| Universe::new(data.into_iter().flatten()))
    .parse(span)
    .expect("valid input");

    assert_eq!(rest.fragment(), &"");

    universe
}

pub fn part1(input: &str) -> u32 {
    let mut u = universe(input.into());
    u.expand();
    u.all_distances()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test_log::test]
    fn test_expand() {
        let mut u = universe("..#\n...\n.#.".into());
        assert_eq!(
            u,
            Universe {
                galaxies: BTreeSet::from([
                    Position { row: 0, col: 2 },
                    Position { row: 2, col: 1 },
                ])
            }
        );

        u.expand();

        assert_eq!(
            u,
            Universe {
                galaxies: BTreeSet::from([
                    Position { row: 0, col: 3 },
                    Position { row: 3, col: 2 },
                ])
            }
        );
    }

    #[test_log::test]
    fn test_part1() {
        info!("Testing...");
        assert_eq!(part1(include_str!("../example.txt")), 374);
    }

    #[test_log::test]
    fn test_parse() {
        assert_eq!(
            universe(include_str!("../example.txt").into()),
            Universe {
                galaxies: BTreeSet::from([
                    Position { row: 0, col: 3 },
                    Position { row: 1, col: 7 },
                    Position { row: 2, col: 0 },
                    Position { row: 4, col: 6 },
                    Position { row: 5, col: 1 },
                    Position { row: 6, col: 9 },
                    Position { row: 8, col: 7 },
                    Position { row: 9, col: 0 },
                    Position { row: 9, col: 4 }
                ])
            }
        );
    }
}
