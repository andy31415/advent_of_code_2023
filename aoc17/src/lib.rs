use std::{ops::Range, iter};

use ndarray::{Array, Array2};
use pathfinding::directed::dijkstra::dijkstra;
use tracing::trace;

/// in what direction are you NOT allowed to go
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

/// A location in a solution
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct SolveLocation {
    deny: Direction,
    row: i32,
    col: i32,
}

#[derive(Debug, PartialEq)]
struct Solver {
    values: Array2<i32>,
}

impl Solver {
    
    fn successors(&self, pos: &SolveLocation) -> Vec<(SolveLocation, usize)> {
        // TODO: grab weight + solve
        // 
        vec![]
    }
    
    fn shortest_path(&self, pos: SolveLocation, goal: (usize, usize)) -> usize {
        let (target_row, target_col) = (goal.0 as i32, goal.1 as i32);

        // start with a particular location and try to reach the goal
        let result = dijkstra(
            &pos,
            |p| self.successors(p),
            |p| (p.row == target_row && p.col == target_col)
        );
        
        result.expect("Dijkstra finds a solution").1
    }
}

fn parse_input(input: &str) -> Array2<i32> {
    let lines = input.split('\n').collect::<Vec<_>>();

    let rows = lines.len();
    let cols = lines.first().map(|l| l.len()).unwrap_or(0);

    let data = lines
        .iter()
        .flat_map(|l| {
            l.chars().into_iter().map(|c| {
                [c].iter()
                    .collect::<String>()
                    .parse::<i32>()
                    .expect("valid input")
            })
        })
        .collect::<Vec<_>>();

    let result = Array::from_shape_vec((rows, cols), data).expect("valid input");

    trace!("Input:\n{:#?}", result);

    result
}

pub fn part1(input: &str) -> usize {
    let solver = Solver {
        values: parse_input(input),
    };

    let a = solver.shortest_path(SolveLocation{ deny: Direction::Up, row: -1, col: 0}, solver.values.dim());
    let b = solver.shortest_path(SolveLocation{ deny: Direction::Left, row: 0, col: -1}, solver.values.dim());
    
    *[a, b].iter().min().expect("have values")
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
