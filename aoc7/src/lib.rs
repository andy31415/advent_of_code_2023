use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, one_of, space1},
    combinator::{opt, value},
    multi,
    sequence::{pair, tuple},
    IResult, Parser,
};
use nom_supreme::ParserExt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Item {
    // A is T is 10, J is 11, Q is 12 and so on
    Card(u8),
    Pair(u8),
    Three(u8),
    Four(u8),
    Five(u8),
}

impl Into<Item> for (u8, i32) {
    fn into(self) -> Item {
        match self.1 {
            1 => Item::Card(self.0),
            2 => Item::Pair(self.0),
            3 => Item::Three(self.0),
            4 => Item::Four(self.0),
            5 => Item::Five(self.0),
            _ => panic!("Invalid count"),
        }
    }
}

pub fn parse_hand(input: &str) -> IResult<&str, Vec<Item>> {
    let (span, mut items) = nom::multi::many_m_n(
        5,
        5,
        alt((
            one_of("0123456789").map(|c| c.to_digit(10).expect("valid digit") as u8),
            value(10, tag("T")),
            value(11, tag("J")),
            value(12, tag("Q")),
            value(13, tag("K")),
            value(14, tag("A")),
        )),
    )
    .parse(input)?;

    items.sort();

    // accumulate items and add them as needed
    let mut result = Vec::<Item>::new();
    let mut previous = (0, 0);
    for x in items.iter() {
        if *x == previous.0 {
            previous.1 = previous.1 + 1;
        } else {
            if previous.1 != 0 {
                result.push(previous.into())
            }
            previous = (*x, 1)
        }
    }
    result.push(previous.into());
    result.sort();
    result.reverse();

    Ok((span, result))
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Ord, Eq)]
pub struct Bid {
    pub hand: Vec<Item>,
    pub value: u32,
}

pub fn parse_bid(input: &str) -> IResult<&str, Bid> {
    tuple((parse_hand, space1, nom::character::complete::u32))
        .map(|(hand, _, value)| Bid { hand, value })
        .parse(input)
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Bid>> {
    multi::many1(parse_bid.terminated(opt(multispace0))).parse(input)
}

pub fn part1_score(input: &str) -> usize {
    let mut bids = parse_input(input).expect("valid input").1;
    bids.sort();
    //bids.reverse();
    bids.iter().enumerate().map(|(rank, bid)| (rank+1) * bid.value as usize).sum()
}

// Stategy:
//   - ordered type (like single)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part1_score(include_str!("../example.txt")), 6440);
    }

    #[test]
    fn check_input_parse() {
        assert_eq!(
            parse_input(include_str!("../example.txt")),
            Ok((
                "",
                vec![
                    Bid {
                        hand: vec![Item::Pair(3), Item::Card(13), Item::Card(10), Item::Card(2)],
                        value: 765
                    },
                    Bid {
                        hand: vec![Item::Three(5), Item::Card(11), Item::Card(10)],
                        value: 684
                    },
                    Bid {
                        hand: vec![Item::Pair(13), Item::Pair(7), Item::Card(6)],
                        value: 28
                    },
                    Bid {
                        hand: vec![Item::Pair(11), Item::Pair(10), Item::Card(13)],
                        value: 220
                    },
                    Bid {
                        hand: vec![Item::Three(12), Item::Card(14), Item::Card(11)],
                        value: 483
                    }
                ]            
            ))
        );
    }

    #[test]
    fn check_parse() {
        assert_eq!(
            parse_hand("AA8AA"),
            Ok(("", vec![Item::Four(14), Item::Card(8)]))
        );
        assert_eq!(
            parse_hand("88Q88"),
            Ok(("", vec![Item::Four(8), Item::Card(12)]))
        );
        assert_eq!(
            parse_hand("2A323"),
            Ok(("", vec![Item::Pair(3), Item::Pair(2), Item::Card(14)]))
        );
        assert_eq!(
            parse_hand("TQ181"),
            Ok((
                "",
                vec![Item::Pair(1), Item::Card(12), Item::Card(10), Item::Card(8)]
            ))
        );
    }

    #[test]
    fn check_order() {
        assert!(Item::Five(10) > Item::Four(10));
        assert!(Item::Four(10) > Item::Three(10));
        assert!(Item::Three(10) > Item::Pair(10));
        assert!(Item::Pair(10) > Item::Card(10));

        assert!(Item::Pair(10) > Item::Pair(9));
    }
}
