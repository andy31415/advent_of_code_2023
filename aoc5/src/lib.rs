use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, space1},
    combinator::value,
    sequence::tuple,
    IResult, Parser,
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct MapRange {
    pub source_start: u32,
    pub dest_start: u32,
    pub len: u32,
}

impl MapRange {
    pub fn try_map(&self, src: u32) -> Option<u32> {
        if src < self.source_start {
            return None;
        }
        if src >= self.source_start + self.len {
            return None;
        }
        return Some(self.dest_start + src - self.source_start);
    }

    pub fn parse(span: &str) -> IResult<&str, MapRange> {
        tuple((
            nom::character::complete::u32,
            space1,
            nom::character::complete::u32,
            space1,
            nom::character::complete::u32,
        ))
        .map(|(dest_start, _, source_start, _, len)| MapRange {
            source_start,
            dest_start,
            len,
        })
        .parse(span)
    }
}

#[derive(PartialEq, Debug, Hash, Clone)]
pub struct MapKey<'a> {
    pub from: &'a str,
    pub to: &'a str,
}

impl MapKey<'_> {
    pub fn parse(span: &str) -> IResult<&str, MapKey<'_>> {
        let (span, from) = alpha1(span)?;
        let (span, _) = tag("-to-")(span)?;
        let (span, to) = alpha1(span)?;

        value(MapKey { from, to }, tuple((space1, tag("map:"))))(span)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_map_key() {
        assert_eq!(
            MapKey::parse("a-to-b map:").expect("valid").1,
            MapKey { from: "a", to: "b" }
        );
        assert_eq!(
            MapKey::parse("soil-to-fertilizer map:").expect("valid").1,
            MapKey {
                from: "soil",
                to: "fertilizer"
            }
        );
    }

    #[test]
    fn test_mapping() {
        let m = MapRange::parse("50 98 2").expect("valid").1;

        assert_eq!(m.try_map(97), None);
        assert_eq!(m.try_map(98), Some(50));
        assert_eq!(m.try_map(99), Some(51));
        assert_eq!(m.try_map(100), None);
    }

    #[test]
    fn parse_range() {
        assert_eq!(
            MapRange::parse("50 98 2").expect("valid").1,
            MapRange {
                source_start: 98,
                dest_start: 50,
                len: 2
            }
        );
        assert_eq!(
            MapRange::parse("88 18 7").expect("valid").1,
            MapRange {
                source_start: 18,
                dest_start: 88,
                len: 7
            }
        );
    }
}
