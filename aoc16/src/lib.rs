use itertools::Itertools;
use nom::{branch::alt, bytes::complete::tag, combinator::value, multi::many1, IResult, Parser};
use nom_locate::LocatedSpan;
use nom_supreme::ParserExt;

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

fn parse_row(input: LocatedSpan<&str>) -> IResult<LocatedSpan<&str>, Vec<(usize, Tile)>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_parse() {
        assert_eq!(
            parse_row(".|...\\....".into()).expect("valid").1,
        vec![
            (1, Tile::Split(SplitDirection::UpDown)),
            (5, Tile::Mirror(MirrorDirection::LeftDownRightUp)),
        ]);
    }
}
