use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::value,
    multi::{many1, separated_list1},
    IResult, Parser,
};
use nom_locate::LocatedSpan;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
enum MirrorDirection {
    LeftDownRightUp,
    LeftUpRightDown,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
enum SplitDirection {
    UpDown,
    LeftRight,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
enum Tile {
    Split(SplitDirection),
    Mirror(MirrorDirection),
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Default)]
struct Beam {
    up: bool,
    right: bool,
    left: bool,
    down: bool,
}

impl Beam {
    fn is_energized(&self) -> bool {
        self.left || self.right || self.up || self.down
    }

    fn is_energized_in_direction(&self, d: Direction) -> bool {
        match d {
            Direction::Left => self.left,
            Direction::Right => self.right,
            Direction::Up => self.up,
            Direction::Down => self.down,
        }
    }

    fn energize(&mut self, d: Direction) {
        match d {
            Direction::Left => self.left = true,
            Direction::Right => self.right = true,
            Direction::Up => self.up = true,
            Direction::Down => self.down = true,
        };
    }
}

struct LightMap {
    energy: HashMap<(usize, usize), Beam>,
    rows: usize,
    cols: usize,

    // faster access for algorightms
    col_mirrors: HashMap<usize, HashMap<usize, Tile>>,
    row_mirrors: HashMap<usize, HashMap<usize, Tile>>,
}

impl LightMap {
    fn new(mirror_map: &Vec<(usize, usize, Tile)>, rows: usize, cols: usize) -> Self {
        let mut col_mirrors: HashMap<usize, HashMap<usize, Tile, _>> = HashMap::new();
        let mut row_mirrors: HashMap<usize, HashMap<usize, Tile, _>> = HashMap::new();

        for (row, col, tile) in mirror_map {
            match row_mirrors.get_mut(row) {
                Some(h) => {
                    h.insert(*col, *tile);
                }
                None => {
                    row_mirrors.insert(*row, HashMap::from_iter([(*col, *tile)]));
                }
            }
            match col_mirrors.get_mut(col) {
                Some(h) => {
                    h.insert(*row, *tile);
                }
                None => {
                    col_mirrors.insert(*row, HashMap::from_iter([(*row, *tile)]));
                }
            }
        }

        Self {
            energy: HashMap::new(),
            col_mirrors,
            row_mirrors,
            rows,
            cols,
        }
    }

    fn send_light(&mut self, row: usize, col: usize, d: Direction) {
        let mut targets = VecDeque::new();
        targets.push_back((row, col, d));

        while let Some((row, col, d)) = targets.pop_front() {
            if self
                .energy
                .get(&(row, col))
                .map(|b| b.is_energized_in_direction(d))
                .unwrap_or(false)
            {
                // if we already energized in this direction
                continue;
            }

            // move from this direction to the next

            // FIXME: implement
        }
    }
}

fn input_row(input: LocatedSpan<&str>) -> IResult<LocatedSpan<&str>, (usize, Vec<(usize, Tile)>)> {
    many1(alt((
        value(Some(Tile::Split(SplitDirection::UpDown)), tag("|")),
        value(Some(Tile::Split(SplitDirection::LeftRight)), tag("-")),
        value(
            Some(Tile::Mirror(MirrorDirection::LeftDownRightUp)),
            tag("\\"),
        ),
        value(
            Some(Tile::Mirror(MirrorDirection::LeftUpRightDown)),
            tag("/"),
        ),
        value(None, tag(".")),
    )))
    .map(|v| {
        (
            v.len(),
            v.iter()
                .enumerate()
                .filter_map(|(idx, x)| x.map(|v| (idx, v)))
                .collect_vec(),
        )
    })
    .parse(input)
}

fn parse_input(input: LocatedSpan<&str>) -> (usize, usize, Vec<(usize, usize, Tile)>) {
    separated_list1(line_ending, input_row)
        .map(|rows| {
            (
                rows.len(),
                rows.first().map(|r| r.0).unwrap_or(0),
                rows.into_iter()
                    .enumerate()
                    .flat_map(|(row, (_, cols))| {
                        cols.into_iter().map(move |(col, t)| (row, col, t))
                    })
                    .collect(),
            )
        })
        .parse(input)
        .expect("Valid input")
        .1
}

pub fn part1(input: &str) -> u32 {
    let (rows, cols, m) = parse_input(input.into());
    let mut map = LightMap::new(&m, rows, cols);

    map.send_light(0, 0, Direction::Right);

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parse() {
        assert_eq!(
            parse_input(".|...\\....\n|.-.\\.....\n..//.|....".into()),
            (
                3,
                10,
                vec![
                    (0, 1, Tile::Split(SplitDirection::UpDown)),
                    (0, 5, Tile::Mirror(MirrorDirection::LeftDownRightUp)),
                    (1, 0, Tile::Split(SplitDirection::UpDown)),
                    (1, 2, Tile::Split(SplitDirection::LeftRight)),
                    (1, 4, Tile::Mirror(MirrorDirection::LeftDownRightUp)),
                    (2, 2, Tile::Mirror(MirrorDirection::LeftUpRightDown)),
                    (2, 3, Tile::Mirror(MirrorDirection::LeftUpRightDown)),
                    (2, 5, Tile::Split(SplitDirection::UpDown)),
                ]
            )
        );
    }

    #[test]
    fn test_row_parse() {
        assert_eq!(
            input_row(".|...\\....".into()).expect("valid").1,
            (
                10,
                vec![
                    (1, Tile::Split(SplitDirection::UpDown)),
                    (5, Tile::Mirror(MirrorDirection::LeftDownRightUp)),
                ]
            )
        );
    }
}
