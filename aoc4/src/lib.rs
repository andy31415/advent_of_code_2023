use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0, space1, self},
    multi::separated_list1,
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
    tuple((
        space0,
        separated_list1(
            space1,
            complete::u32,
        ),
    ))
    .map(|(_, digits)| digits)
    .parse(data)
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
        match self.wins().checked_sub(1) {
            None => 0,
            Some(cnt) => 1 << cnt,
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

pub fn part_2_sum_cards(lines: &str) -> usize {
    let cards = Card::parse_many(lines).expect("valid input");
    let mut counts: Vec<usize> = Vec::with_capacity(cards.len());

    // buy one card each time
    counts.resize(cards.len(), 1);

    for i in 0..cards.len() {
        let card = cards.get(i).expect("valid card");
        let count = *counts.get(i).expect("valid index");

        for n in 1..=card.wins() {
            let idx = i + n;
            if let Some(cnt) = counts.get_mut(idx) {
                *cnt += count;
            }
        }
    }

    counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

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
    fn test_part2() {
        assert_eq!(part_2_sum_cards(include_str!("../example.txt")), 30);
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

    #[rstest]
    #[case("Card 1: 1 2 3 | 4 5 6", 0)]
    #[case("Card 1: 1 2 3 | 1 5 6", 1)]
    #[case("Card 1: 1 2 3 | 1 2 6", 2)]
    #[case("Card 1: 1 2 3 | 1 2 3", 4)]
    #[case("Card 1: 1 2 3 4 | 1 2 3 4", 8)]
    #[case("Card 1: 1 2 3 4 | 6 4 10 2", 2)]
    fn test_points(#[case] card: &str, #[case] points: usize) {
        assert_eq!(Card::parse(card).expect("valud").points(), points);
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
