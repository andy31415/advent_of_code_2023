use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0},
    multi::many1,
    sequence::tuple,
    IResult, Parser,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Card {
    pub num: u32,
    pub winning: Vec<u32>,
    pub actual: Vec<u32>,
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
    pub fn parse(line: &str) -> Result<Self, String> {
        let mut parser = tuple((
            tag("Card "),
            digit1,
            tag(":"),
            spaced_numbers,
            tag(" | "),
            spaced_numbers,
        ))
        .map(|(_, id, _, winning, _, actual)| Card {
            num: id.parse::<u32>().expect("valid digits"),
            winning,
            actual,
        });

        match parser.parse(line) {
            Err(e) => Err(format!("Error parsing: {:?}", e).into()),
            Ok(v) => Ok(v.1),
        }
    }
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
    fn test_parse_card() {
        assert_eq!(
            Card::parse("Card 1: 1 2 3 | 4 5 6"),
            Ok(Card {
                num: 1,
                winning: vec![1, 2, 3],
                actual: vec![4, 5, 6],
            })
        );
        
        assert_eq!(
            Card::parse("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            Ok(Card {
                num: 4,
                winning: vec![41, 92, 73, 84, 69],
                actual: vec![59, 84, 76, 51, 58, 5, 54, 83],
            })
        );
    }
}
