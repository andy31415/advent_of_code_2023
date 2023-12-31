use std::{
    collections::{HashMap, VecDeque},
    fmt::Write,
};

use rayon::prelude::*;

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
use tracing::{info, trace};

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

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Tile::Split(SplitDirection::LeftRight) => '-',
            Tile::Split(SplitDirection::UpDown) => '|',
            Tile::Mirror(MirrorDirection::LeftDownRightUp) => '/',
            Tile::Mirror(MirrorDirection::LeftUpRightDown) => '\\',
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
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
    fn display_char(&self) -> char {
        let mut cnt = 0;
        if self.left {
            cnt += 1;
        }
        if self.right {
            cnt += 1;
        }
        if self.up {
            cnt += 1;
        }
        if self.down {
            cnt += 1;
        }

        match cnt {
            0 => '.',
            1 if self.left => '←',
            1 if self.right => '→',
            1 if self.up => '↑',
            1 if self.down => '↓',
            2 if self.left && self.right => '⇆',
            2 if self.up && self.down => '⇅',
            2 => '2',
            3 => '3',
            4 => '4',
            _ => unreachable!(),
        }
    }

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

#[derive(Clone)]
struct LightMap {
    map: HashMap<(usize, usize), Tile>,
    energy: HashMap<(usize, usize), Beam>,
    rows: usize,
    cols: usize,
}

impl std::fmt::Display for LightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                match self.map.get(&(row, col)) {
                    Some(t) => f.write_fmt(format_args!("{}", t))?,
                    None => f.write_char('.')?,
                }
            }

            f.write_str("    |    ")?;

            for col in 0..self.cols {
                f.write_char(match self.energy.get(&(row, col)) {
                    Some(b) => b.display_char(),
                    None => '.',
                })?;
            }

            f.write_char('\n')?
        }
        Ok(())
    }
}

impl LightMap {
    fn new(mirror_map: &Vec<(usize, usize, Tile)>, rows: usize, cols: usize) -> Self {
        Self {
            energy: HashMap::new(),
            map: mirror_map.iter().map(|(r, c, t)| ((*r, *c), *t)).collect(),
            rows,
            cols,
        }
    }

    fn move_towards(&self, row: usize, col: usize, d: Direction) -> Option<(usize, usize)> {
        match d {
            Direction::Up if row > 0 => Some((row - 1, col)),
            Direction::Down if row + 1 < self.rows => Some((row + 1, col)),
            Direction::Left if col > 0 => Some((row, col - 1)),
            Direction::Right if col + 1 < self.cols => Some((row, col + 1)),
            _ => None,
        }
    }

    /// Beams the light at the specified row, column and direction
    /// returns where the light goes from there
    fn beam_step(
        &mut self,
        row: usize,
        col: usize,
        d: Direction,
    ) -> Vec<(usize, usize, Direction)> {
        let map_element = self.map.get(&(row, col));

        // Energize current tile
        match self.energy.get_mut(&(row, col)) {
            Some(v) => v.energize(d),
            None => {
                self.energy.insert((row, col), {
                    let mut b = Beam::default();
                    b.energize(d);
                    b
                });
            }
        }

        // Figure out where to go with the beams
        let mut directions = Vec::new();
        match map_element {
            None => directions.push(d),
            Some(tile) => match (tile, d) {
                (Tile::Split(SplitDirection::LeftRight), Direction::Left)
                | (Tile::Split(SplitDirection::LeftRight), Direction::Right)
                | (Tile::Split(SplitDirection::UpDown), Direction::Up)
                | (Tile::Split(SplitDirection::UpDown), Direction::Down) => {
                    directions.push(d);
                }

                (Tile::Split(SplitDirection::UpDown), Direction::Left)
                | (Tile::Split(SplitDirection::UpDown), Direction::Right) => {
                    directions.push(Direction::Up);
                    directions.push(Direction::Down);
                }

                (Tile::Split(SplitDirection::LeftRight), Direction::Up)
                | (Tile::Split(SplitDirection::LeftRight), Direction::Down) => {
                    directions.push(Direction::Left);
                    directions.push(Direction::Right);
                }

                (Tile::Mirror(MirrorDirection::LeftDownRightUp), Direction::Left) => {
                    directions.push(Direction::Down)
                }

                (Tile::Mirror(MirrorDirection::LeftUpRightDown), Direction::Left) => {
                    directions.push(Direction::Up)
                }

                (Tile::Mirror(MirrorDirection::LeftDownRightUp), Direction::Right) => {
                    directions.push(Direction::Up)
                }
                (Tile::Mirror(MirrorDirection::LeftUpRightDown), Direction::Right) => {
                    directions.push(Direction::Down)
                }

                (Tile::Mirror(MirrorDirection::LeftDownRightUp), Direction::Up) => {
                    directions.push(Direction::Right)
                }
                (Tile::Mirror(MirrorDirection::LeftUpRightDown), Direction::Up) => {
                    directions.push(Direction::Left)
                }

                (Tile::Mirror(MirrorDirection::LeftDownRightUp), Direction::Down) => {
                    directions.push(Direction::Left)
                }
                (Tile::Mirror(MirrorDirection::LeftUpRightDown), Direction::Down) => {
                    directions.push(Direction::Right)
                }
            },
        }

        directions
            .iter()
            .filter_map(|d| self.move_towards(row, col, *d).map(|(r, c)| (r, c, *d)))
            .collect()
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

            for s in self.beam_step(row, col, d) {
                targets.push_back(s);
            }
            trace!("AFTER {:?}:\n{}", (row, col, d), &self);
        }
    }

    // Runs energy calculation but resets enegy map back
    fn energy_for_beam(&self, row: usize, col: usize, d: Direction) -> usize {
        let mut copy = self.clone();
        copy.energy.clear();
        copy.send_light(row, col, d);
        let energy = copy.count_energy();
        energy
    }

    // RETURNS: row, col, energy
    fn max_energy(&mut self) -> (usize, usize, Direction, usize) {
        (0..self.rows)
            .map(|r| (r, 0, Direction::Right))
            .chain((0..self.cols).map(|c| (0, c, Direction::Down)))
            .collect_vec()
            .par_iter()
            .map(|(r, c, d)| (*r, *c, *d, self.energy_for_beam(*r, *c, *d)))
            .max_by(|a, b| a.3.cmp(&b.3))
            .expect("Has value")

    }

    fn count_energy(&self) -> usize {
        self.energy.iter().filter(|(_, b)| b.is_energized()).count()
    }
}

fn input_row(input: LocatedSpan<&str>) -> IResult<LocatedSpan<&str>, (usize, Vec<(usize, Tile)>)> {
    many1(alt((
        value(Some(Tile::Split(SplitDirection::UpDown)), tag("|")),
        value(Some(Tile::Split(SplitDirection::LeftRight)), tag("-")),
        value(
            Some(Tile::Mirror(MirrorDirection::LeftDownRightUp)),
            tag("/"),
        ),
        value(
            Some(Tile::Mirror(MirrorDirection::LeftUpRightDown)),
            tag("\\"),
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

pub fn part1(input: &str) -> usize {
    let (rows, cols, m) = parse_input(input.into());
    let mut map = LightMap::new(&m, rows, cols);
    info!("BEFORE:\n{}", &map);
    map.send_light(0, 0, Direction::Right);
    info!("AFTER:\n{}", &map);
    map.count_energy()
}

pub fn part2(input: &str) -> usize {
    let (rows, cols, m) = parse_input(input.into());
    let mut map = LightMap::new(&m, rows, cols);
    map.max_energy().3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../example.txt")), 46);
    }

    #[test_log::test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../example.txt")), 51);
    }

    #[test]
    fn test_input_parse() {
        assert_eq!(
            parse_input(".|...\\....\n|.-.\\.....\n..//.|....".into()),
            (
                3,
                10,
                vec![
                    (0, 1, Tile::Split(SplitDirection::UpDown)),
                    (0, 5, Tile::Mirror(MirrorDirection::LeftUpRightDown)),
                    (1, 0, Tile::Split(SplitDirection::UpDown)),
                    (1, 2, Tile::Split(SplitDirection::LeftRight)),
                    (1, 4, Tile::Mirror(MirrorDirection::LeftUpRightDown)),
                    (2, 2, Tile::Mirror(MirrorDirection::LeftDownRightUp)),
                    (2, 3, Tile::Mirror(MirrorDirection::LeftDownRightUp)),
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
                    (5, Tile::Mirror(MirrorDirection::LeftUpRightDown)),
                ]
            )
        );
    }
}
