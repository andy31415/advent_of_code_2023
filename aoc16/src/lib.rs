use std::collections::HashMap;

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

fn input_row(input: LocatedSpan<&str>) -> IResult<LocatedSpan<&str>, Vec<(usize, Tile)>> {
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
        v.iter()
            .enumerate()
            .filter_map(|(idx, x)| x.map(|v| (idx, v)))
            .collect_vec()
    })
    .parse(input)
}

fn parse_input(input: LocatedSpan<&str>) -> HashMap<(usize, usize), Tile> {
    separated_list1(line_ending, input_row)
        .map(|rows| {
            rows.into_iter()
                .enumerate()
                .flat_map(|(row, row_pos)| row_pos.into_iter().map(move |(col, t)| ((row, col), t)))
                .collect()
        })
        .parse(input)
        .expect("Valid input")
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parse() {
        assert_eq!(
            parse_input(".|...\\....\n|.-.\\.....\n..//.|....".into()),
            [
                ((0, 1), Tile::Split(SplitDirection::UpDown)),
                ((0, 5), Tile::Mirror(MirrorDirection::LeftDownRightUp)),
                ((1, 0), Tile::Split(SplitDirection::UpDown)),
                ((1, 2), Tile::Split(SplitDirection::LeftRight)),
                ((1, 4), Tile::Mirror(MirrorDirection::LeftDownRightUp)),
                ((2, 2), Tile::Mirror(MirrorDirection::LeftUpRightDown)),
                ((2, 3), Tile::Mirror(MirrorDirection::LeftUpRightDown)),
                ((2, 5), Tile::Split(SplitDirection::UpDown)),
            ]
            .into()
        );
    }

    #[test]
    fn test_row_parse() {
        assert_eq!(
            input_row(".|...\\....".into()).expect("valid").1,
            vec![
                (1, Tile::Split(SplitDirection::UpDown)),
                (5, Tile::Mirror(MirrorDirection::LeftDownRightUp)),
            ]
        );
    }
}
