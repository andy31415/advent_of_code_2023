use ndarray::Array2;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::value,
    multi::{many1, separated_list1},
    IResult, Parser,
};

#[derive(Debug, PartialEq)]
pub struct Puzzle {
    pub data: Array2<bool>,
}

fn puzzle(input: &str) -> IResult<&str, Puzzle> {
    separated_list1(
        line_ending,
        many1(alt((value(false, tag(".")), value(true, tag("#"))))),
    )
    .map(|data| {
        let cols = data.iter().next().expect("Non-empty puzle").len();
        let rows = data.len();

        assert!(data.iter().all(|v| v.len() == cols));

        let raw = data.into_iter().flatten().collect::<Vec<_>>();

        Puzzle {
            data: Array2::from_shape_vec((rows, cols), raw).expect("vector is the right size"),
        }
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use ndarray::array;

    use super::*;

    #[test]
    fn test_parse_puzzle() {
        assert_eq!(
            puzzle(
                "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
            )
            .expect("valid input")
            .1
            .data,
            array![
                [true, false, true, true, false, false, true, true, false],
                [false, false, true, false, true, true, false, true, false],
                [true, true, false, false, false, false, false, false, true],
                [true, true, false, false, false, false, false, false, true],
                [false, false, true, false, true, true, false, true, false],
                [false, false, true, true, false, false, true, true, false],
                [true, false, true, false, true, true, false, true, false]
            ]
        );
    }
}
