use std::fmt::Write;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, one_of, space1},
    combinator::{opt, value},
    multi,
    sequence::tuple,
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

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

impl Item {
    pub fn display_char(&self) -> char {
        let v = match self {
            Item::Card(x) => x,
            Item::Pair(x) => x,
            Item::Three(x) => x,
            Item::Four(x) => x,
            Item::Five(x) => x,
        };
        match v {
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            10 => 'T',
            11 => 'J',
            12 => 'Q',
            13 => 'K',
            14 => 'A',
            _ => panic!("Invalid value: {}", v),
        }
    }

    pub fn count(&self) -> u8 {
        match self {
            Item::Card(_) => 1,
            Item::Pair(_) => 2,
            Item::Three(_) => 3,
            Item::Four(_) => 4,
            Item::Five(_) => 5,
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 1..=self.count() {
            f.write_char(self.display_char())?;
        }
        Ok(())
    }
}

impl From<(u8, i32)> for Item {
    fn from(val: (u8, i32)) -> Self {
        match val.1 {
            1 => Item::Card(val.0),
            2 => Item::Pair(val.0),
            3 => Item::Three(val.0),
            4 => Item::Four(val.0),
            5 => Item::Five(val.0),
            _ => panic!("Invalid count"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hand {
    cards: Vec<u8>, // cards in order
    items: Vec<Item>,
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in self.items.iter() {
            f.write_fmt(format_args!("{}", x))?;
        }
        f.write_str("(")?;
        for x in self.cards.iter() {
            f.write_fmt(format_args!("{}:", x))?;
        }
        f.write_str(")")?;
        Ok(())
    }
}

impl Hand {
    pub fn hand_type(&self) -> Type {
        match self.items.get(0).expect("valid hand") {
            Item::Five(_) => Type::FiveOfAKind,
            Item::Four(_) => Type::FourOfAKind,
            Item::Three(_) => match self.items.get(1).expect("valid input") {
                Item::Pair(_) => Type::FullHouse,
                _ => Type::ThreeOfAKind,
            }
            Item::Pair(_) => match self.items.get(1).expect("valid input") {
                Item::Pair(_) => Type::TwoPair,
                _ => Type::OnePair,
            }
            Item::Card(_) => Type::HighCard,
        }
    }
    
    
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type() == other.hand_type() {
            // NOT a card game: order is based on cards that are dealt
            return self.cards.cmp(&other.cards);
        }

        // if different lenght, shorter wins
        // this makes:
        //   two pair win over one pair
        //   full wins over three of a kind
        //  Type        | Length
        // -------------+-----------
        //  5 of a kind | 1
        //  4 of a kind | 2
        //  Full house  | 2
        //  3 of a kind | 3
        //  two pair    | 3
        //  one pair    | 4
        //  high card   | 5
        if self.items.len() < other.items.len() {
            return std::cmp::Ordering::Greater;
        } else if self.items.len() > other.items.len() {
            return std::cmp::Ordering::Less;
        }
        self.items.cmp(&other.items)
    }
}

pub fn parse_hand(input: &str) -> IResult<&str, Hand> {
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
    
    let cards = items.clone();

    items.sort();

    // accumulate items and add them as needed
    let mut result = Vec::<Item>::new();
    let mut previous = (0, 0);
    for x in items.iter() {
        if *x == previous.0 {
            previous.1 += 1;
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

    Ok((span, Hand { cards, items: result }))
}

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct Bid {
    pub hand: Hand,
    pub value: u32,
}

impl std::fmt::Display for Bid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Bid: {} -> {}", self.hand, self.value))
    }
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
    let (left, mut bids) = parse_input(input).expect("valid input");
    assert_eq!(left, "");

    // smallest hand goes first
    bids.sort();
    bids.iter()
        .enumerate()
        .map(|(rank, bid)| (rank + 1) * bid.value as usize)
        .sum()
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
                        hand: Hand {
                            items: vec![
                                Item::Pair(3),
                                Item::Card(13),
                                Item::Card(10),
                                Item::Card(2)
                            ],
                            cards: vec![3,2,10,3,13],
                        },
                        value: 765
                    },
                    Bid {
                        hand: Hand {
                            items: vec![Item::Three(5), Item::Card(11), Item::Card(10)],
                            cards: vec![10,5,5,11,5],
                        },
                        value: 684
                    },
                    Bid {
                        hand: Hand {
                            items: vec![Item::Pair(13), Item::Pair(7), Item::Card(6)],
                            cards: vec![13,13,6,7,7],
                        },
                        value: 28
                    },
                    Bid {
                        hand: Hand {
                            items: vec![Item::Pair(11), Item::Pair(10), Item::Card(13)],
                            cards: vec![13,10,11,11,10]
                        },
                        value: 220
                    },
                    Bid {
                        hand: Hand {
                            items: vec![Item::Three(12), Item::Card(14), Item::Card(11)],
                            cards: vec![12,12,12,11,14]
                        },
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
            Ok((
                "",
                Hand {
                    items: vec![Item::Four(14), Item::Card(8)],
                    cards: vec![14,14,8,14,14],
                }
            ))
        );
        assert_eq!(
            parse_hand("TQ181"),
            Ok((
                "",
                Hand {
                    items: vec![Item::Pair(1), Item::Card(12), Item::Card(10), Item::Card(8)],
                    cards: vec![10,12,1,8,1]
                }
            ))
        );
    }

    fn assert_ordered(a: &str, b: &str) {
        let a = parse_hand(a).expect("valid").1;
        let b = parse_hand(b).expect("valid").1;
        if a < b {
            panic!("{} > {} failed", a, b);
        } 
        if b > a {
            panic!("{} < {} failed", b, a);
        } 
    }

    #[test]
    fn mix_order() {
        assert_ordered("AAAAA", "AA8AA");
        assert_ordered("AA8AA", "23332");
        assert_ordered("23332", "TTT98");
        assert_ordered("TTT98", "23432");
        assert_ordered("23432", "A23A4");
        assert_ordered("A23A4", "23456");

        // change things up
        assert_ordered("22345", "AKQT9");
        assert_ordered("22334", "AAKQT");
        assert_ordered("22234", "AAKKQ");
        assert_ordered("33344", "AAAKQ");
        assert_ordered("22223", "AAAKK");
        assert_ordered("22222", "AAAAK");
        // same type
        assert_ordered("A2234", "93345");
        assert_ordered("A2233", "9AAKK");
        assert_ordered("A3334", "9AAA5");
        assert_ordered("A3333", "9AAAA");
    }

    #[test]
    fn more_order() {
        let b1 = parse_hand("AK642").expect("valid").1;
        let b2 = parse_hand("TTJ43").expect("valid").1;
        assert!(b2 > b1);

        // two pair better than one pair
        let b1 = parse_hand("AA234").expect("valid").1;
        let b2 = parse_hand("22335").expect("valid").1;
        assert!(b2 > b1);
        assert!(b1 < b2);

        // Full house wins over 3 of a kind
        let b1 = parse_hand("33344").expect("valid").1;
        let b2 = parse_hand("AAAKQ").expect("valid").1;
        dbg!(&b1);
        dbg!(&b2);
        assert!(b1 > b2);
        assert!(b2 < b1);
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
