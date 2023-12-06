use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, space1},
    combinator::value,
    sequence::tuple,
    IResult, Parser, multi::{separated_list1, many0, many1},
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
        Some(self.dest_start + src - self.source_start)
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

#[derive(PartialEq, Debug, Hash, Clone, Eq)]
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

#[derive(Clone, Default, Debug, PartialEq)]
pub struct InputData<'a> {
    seeds: Vec<u32>,
    maps: HashMap<MapKey<'a>, Vec<MapRange>>,
}

impl InputData<'_> {
    
    pub fn get_map_from(&self, state: &str) -> Option<&MapKey<'_>> {
        for k in self.maps.keys() {
            if k.from == state {
                return Some(k)
            }
        }
        None
        
        
    }
    
    pub fn place(&self, mut value: u32, name: &str) -> u32 {
        let mut state = "seed";
        while state != name {
            let key = self.get_map_from(state).expect("valid input");
            for m in self.maps.get(key).expect("valid input") {
                if let Some(new_pos) = m.try_map(value) {
                    value = new_pos;
                    break;
                }
            }
            // not mapped preserves location
            state = key.to;
        }
        
        value
    }
    
    pub fn parse(span: &str) -> IResult<&str, InputData> {
        // start with seeds map
        let (span, _) = tuple((tag("seeds:"), space1)).parse(span)?;
        let (span, seeds) = separated_list1(space1, nom::character::complete::u32).parse(span)?;
        let (span, _) = tag("\n").parse(span)?;
        
        let (span, mappings) = many0(
            tuple((
            tag("\n"),
            MapKey::parse,
            tag("\n"),
            many1(tuple((MapRange::parse, tag("\n"))).map(|(r,_)| r))
        )).map(|(_,key,_, items)| (key, items))
        )
        .parse(span)?;
        
        let maps = HashMap::from_iter(mappings);
        Ok((span, InputData{seeds, maps}))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example_map() {
        let r = InputData::parse(include_str!("../example.txt")).expect("valid input").1;
        assert_eq!(r.place(79, "location"), 82);
        assert_eq!(r.place(14, "location"), 43);
        assert_eq!(r.place(55, "location"), 86);
        assert_eq!(r.place(13, "location"), 35);
    }

    #[test]
    fn test_parse_input() {
        let r = InputData::parse(include_str!("../example.txt")).expect("valid input").1;
        
        assert_eq!(r.seeds, [79, 14, 55, 13]);
        assert_eq!(r.maps.len(), 7);
        
        assert_eq!(InputData::parse(include_str!("../example.txt")).expect("valid input").0, "");
        assert_eq!(InputData::parse(include_str!("../input.txt")).expect("valid input").0, "");
    }

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
