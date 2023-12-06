use nom::{
    character::{self, complete::space1},
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
            character::complete::u32,
            space1,
            character::complete::u32,
            space1,
            character::complete::u32,
        ))
        .map(|(dest_start, _, source_start, _, len)| MapRange {
            source_start,
            dest_start,
            len,
        })
        .parse(span)
    }
}

#[cfg(test)]
mod tests {
    use crate::MapRange;

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
