use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0, space1},
    multi::many1,
    sequence::tuple,
    IResult, Parser,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Card {
    pub num: u32,
    pub winning: HashSet<u32>,
    pub actual: HashSet<u32>,
}

fn spaced_numbers(data: &str) -> IResult<&str, Vec<u32>> {
    many1(
        tuple((
            space0,                                                              // at start
            digit1.map(|span: &str| span.parse::<u32>().expect("Valid digits")), // and numbers after
        ))
        .map(|(_, digits)| digits),
    )(data)
}

impl Card {
    pub fn parse_many(lines: &str) -> Result<Vec<Self>, String> {
        lines.split('\n').map(Card::parse).collect()
    }

    pub fn parse(line: &str) -> Result<Self, String> {
        let mut parser = tuple((
            tag("Card"),
            space1,
            digit1,
            tag(":"),
            spaced_numbers,
            tag(" | "),
            spaced_numbers,
        ))
        .map(|(_, _, id, _, winning, _, actual)| Card {
            num: id.parse::<u32>().expect("valid digits"),
            winning: HashSet::from_iter(winning),
            actual: HashSet::from_iter(actual),
        });

        match parser.parse(line) {
            Err(e) => Err(format!("Error parsing: {:?}", e)),
            Ok(v) => Ok(v.1),
        }
    }

    pub fn wins(&self) -> usize {
        self.winning.intersection(&self.actual).count()
    }

    pub fn points(&self) -> usize {
        match self.wins() {
            0 => 0,
            cnt => {
                let mut result = 1;
                for _ in 1..cnt {
                    result *= 2;
                }
                result
            }
        }
    }
}

pub fn part_1_add_points(lines: &str) -> usize {
    Card::parse_many(lines)
        .expect("valid input")
        .iter()
        .map(Card::points)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parse_spaced_numbers() {
        assert_eq!(spaced_numbers("1 2 3 4"), Ok(("", vec![1, 2, 3, 4])));
        assert_eq!(spaced_numbers("1 2 | a b c"), Ok((" | a b c", vec![1, 2])));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_1_add_points(include_str!("../example.txt")), 13);
    }

    #[test]
    fn test_parse_many() {
        let cards = Card::parse_many(include_str!("../example.txt")).expect("Valid example");

        assert_eq!(cards.len(), 6);
        assert_eq!(
            cards.get(1),
            Some(&Card {
                num: 2,
                winning: HashSet::from_iter(vec![13, 32, 20, 16, 61]),
                actual: HashSet::from_iter(vec![61, 30, 68, 82, 17, 32, 24, 19]),
            })
        );

        assert_eq!(cards.get(0).expect("Valid").points(), 8);
        assert_eq!(cards.get(1).expect("Valid").points(), 2);
        assert_eq!(cards.get(2).expect("Valid").points(), 2);
        assert_eq!(cards.get(3).expect("Valid").points(), 1);
        assert_eq!(cards.get(4).expect("Valid").points(), 0);
        assert_eq!(cards.get(5).expect("Valid").points(), 0);
    }

    #[test]
    fn test_parse_card() {
        assert_eq!(
            Card::parse("Card 1: 1 2 3 | 4 5 6"),
            Ok(Card {
                num: 1,
                winning: HashSet::from_iter(vec![1, 2, 3]),
                actual: HashSet::from_iter(vec![4, 5, 6]),
            })
        );

        assert_eq!(
            Card::parse("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            Ok(Card {
                num: 4,
                winning: HashSet::from_iter(vec![41, 92, 73, 84, 69]),
                actual: HashSet::from_iter(vec![59, 84, 76, 51, 58, 5, 54, 83]),
            })
        );
    }
}
