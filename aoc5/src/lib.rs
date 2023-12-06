use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, space1},
    combinator::value,
    multi::{many0, many1, separated_list1},
    sequence::tuple,
    IResult, Parser,
};

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub struct MapRange {
    source_start: i64,
    source_end: i64, // NOT includisve
    dest_start: i64,
}

trait Remapper {
    fn try_map(&self, src: i64) -> Option<i64>;
}

/// Vectors ALWAYS succeed mapping
impl Remapper for Vec<MapRange> {
    fn try_map(&self, src: i64) -> Option<i64> {
        for m in self {
            if let Some(pos) = m.try_map(src) {
                return Some(pos);
            }
        }
        Some(src)
    }
}

impl MapRange {
    /// Maps a input number to the output value
    pub fn try_map(&self, src: i64) -> Option<i64> {
        if src >= self.source_start && src < self.source_end {
            Some(self.dest_start + src - self.source_start)
        } else {
            None
        }
    }

    /// Transforms an input range into one or more output ranges
    pub fn transform(&self, inputs: &Vec<MapRange>) -> Vec<MapRange> {
        let mut split_positions = Vec::new();
        split_positions.push(self.source_start);
        split_positions.push(self.source_end);

        for t in inputs.iter() {
            let t_start = self.source_start - self.dest_start + t.source_start;
            let t_end = t_start + t.source_end - t.source_start;

            if t_start > self.source_start && t_start < self.source_end {
                split_positions.push(t_start);
            }
            if t_end > self.source_start && t_end < self.source_end {
                split_positions.push(t_end);
            }
        }
        split_positions.sort();
        
        split_positions
            .as_slice()
            .windows(2)
            .map(|chunk| {
                let start = chunk[0];
                let end = chunk[1];

                let target = self.try_map(start).unwrap_or(start);

                MapRange::from_start_end(start, end, inputs.try_map(target).unwrap_or(target))
            })
            .collect()
    }

    /// Constructor for start/end
    pub fn from_start_end(source_start: i64, source_end: i64, dest_start: i64) -> Self {
        Self {
            source_start,
            source_end,
            dest_start,
        }
    }

    /// Constructs a map range given from/to/len
    pub fn from_to_len(from: i64, to: i64, len: i64) -> Self {
        Self {
            source_start: from,
            source_end: from + len,
            dest_start: to,
        }
    }

    pub fn parse(span: &str) -> IResult<&str, MapRange> {
        tuple((
            nom::character::complete::i64,
            space1,
            nom::character::complete::i64,
            space1,
            nom::character::complete::i64,
        ))
        .map(|(dest_start, _, source_start, _, len)| {
            MapRange::from_to_len(source_start, dest_start, len)
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
    seeds: Vec<i64>,
    maps: HashMap<MapKey<'a>, Vec<MapRange>>,
}

impl InputData<'_> {
    pub fn get_map_from(&self, state: &str) -> Option<&MapKey<'_>> {
        for k in self.maps.keys() {
            if k.from == state {
                return Some(k);
            }
        }
        None
    }

    pub fn place(&self, mut value: i64, name: &str) -> i64 {
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
        let (span, seeds) = separated_list1(space1, nom::character::complete::i64).parse(span)?;
        let (span, _) = tag("\n").parse(span)?;

        let (span, mappings) = many0(
            tuple((
                tag("\n"),
                MapKey::parse,
                tag("\n"),
                many1(tuple((MapRange::parse, tag("\n"))).map(|(r, _)| r)),
            ))
            .map(|(_, key, _, items)| (key, items)),
        )
        .parse(span)?;

        let maps = HashMap::from_iter(mappings);
        Ok((span, InputData { seeds, maps }))
    }
}

pub fn part_1_min(input: &str) -> i64 {
    let data = InputData::parse(input).expect("good input").1;
    data.seeds
        .iter()
        .map(|s| data.place(*s, "location"))
        .min()
        .unwrap()
}

pub fn part_2_min(input: &str) -> i64 {
    let data = InputData::parse(input).expect("good input").1;

    // every data seed is an identity map ....
    // //
    let mut maps = data
        .seeds
        .chunks(2)
        .map(|w| MapRange::from_to_len(w[0], w[0], w[1]))
        .collect::<Vec<_>>();

    let mut state = "seed";
    while state != "location" {
        // find the next step
        let key = data.get_map_from(state).expect("valid input");
        maps = maps
            .iter()
            .flat_map(|m| m.transform(&data.maps.get(key).expect("valid input")))
            .collect();
        maps.sort();
        state = key.to;
    }

    // minimum will be at one of the starts
    maps.iter()
        .map(|m| m.try_map(m.source_start).unwrap_or(m.source_start))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(part_1_min(include_str!("../example.txt")), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2_min(include_str!("../example.txt")), 46);
    }

    #[test]
    fn test_chunk_map() {
        assert_eq!(
            MapRange::from_start_end(10, 20, 100)
                .transform(&vec![MapRange::from_start_end(5, 8, 30)]),
            vec![MapRange::from_start_end(10, 20, 100)],
        );

        assert_eq!(
            MapRange::from_start_end(10, 20, 5)
                .transform(&vec![MapRange::from_start_end(5, 30, 300)]),
            vec![MapRange::from_start_end(10, 20, 300)],
        );

        assert_eq!(
            MapRange::from_start_end(10, 20, 15)
                .transform(&vec![MapRange::from_start_end(5, 30, 300)]),
            vec![MapRange::from_start_end(10, 20, 310)],
        );

        // 10 -> 50
        // 20 -> 60 -> 10
        // 30 -> 70 -> 20 // by one
        // 40 -> 90
        assert_eq!(
            MapRange::from_start_end(10, 40, 50)
                .transform(&vec![MapRange::from_start_end(60, 70, 10)]),
            vec![
                MapRange::from_start_end(10, 20, 50),
                MapRange::from_start_end(20, 30, 10),
                MapRange::from_start_end(30, 40, 70),
            ],
        );

        // 25 -> 65 -> 15
        // 30 -> 70 -> 20 // by one
        // 40 -> 80
        assert_eq!(
            MapRange::from_start_end(25, 40, 65)
                .transform(&vec![MapRange::from_start_end(60, 70, 10)]),
            vec![
                MapRange::from_start_end(25, 30, 15),
                MapRange::from_start_end(30, 40, 70),
            ],
        );
    }

    #[test]
    fn test_example_map() {
        let r = InputData::parse(include_str!("../example.txt"))
            .expect("valid input")
            .1;
        assert_eq!(r.place(79, "location"), 82);
        assert_eq!(r.place(14, "location"), 43);
        assert_eq!(r.place(55, "location"), 86);
        assert_eq!(r.place(13, "location"), 35);
    }

    #[test]
    fn test_parse_input() {
        let r = InputData::parse(include_str!("../example.txt"))
            .expect("valid input")
            .1;

        assert_eq!(r.seeds, [79, 14, 55, 13]);
        assert_eq!(r.maps.len(), 7);

        assert_eq!(
            InputData::parse(include_str!("../example.txt"))
                .expect("valid input")
                .0,
            ""
        );
        assert_eq!(
            InputData::parse(include_str!("../input.txt"))
                .expect("valid input")
                .0,
            ""
        );
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
            MapRange::from_to_len(98, 50, 2)
        );
        assert_eq!(
            MapRange::parse("88 18 7").expect("valid").1,
            MapRange::from_to_len(18, 88, 7)
        );
    }
}
