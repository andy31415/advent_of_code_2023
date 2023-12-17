use ndarray::{Array, Array2};
use tracing::trace;

#[derive(Debug, PartialEq)]
struct Solver {
    values: Array2<usize>,
}

fn parse_input(input: &str) -> Array2<usize> {
    let lines = input.split('\n').collect::<Vec<_>>();

    let rows = lines.len();
    let cols = lines.first().map(|l| l.len()).unwrap_or(0);

    let data = lines
        .iter()
        .flat_map(|l| {
            l.chars().into_iter().map(|c| {
                [c].iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .expect("valid input")
            })
        })
        .collect::<Vec<_>>();

    let result = Array::from_shape_vec((rows, cols), data).expect("valid input");
    
    trace!("Input:\n{:#?}", result);
    
    result
}

pub fn part1(input: &str) -> usize {
    let solver = Solver{values: parse_input(input)};
    // TODO: implement
    0
}

pub fn part2(input: &str) -> usize {
    // TODO: implement
    0
}

#[cfg(test)]
mod tests {
    use ndarray::arr2;

    use super::*;

    #[test_log::test]
    fn test_input_parse() {
        assert_eq!(
            parse_input("123\n321\n888\n223"),
            arr2(&[[1, 2, 3], [3, 2, 1], [8, 8, 8], [2, 2, 3]])
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../example.txt")), 102);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../example.txt")), 0);
    }
}
