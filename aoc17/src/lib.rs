use std::panic::Location;

use ndarray::{Array, Array2};
use pathfinding::directed::dijkstra::dijkstra;
use tracing::{info, trace};

/// in what direction are you NOT allowed to go
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn invert(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

/// A location in a solution
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
struct SolveLocation {
    from_direction: Direction,
    from_len: usize,
    row: i32,
    col: i32,
}

#[derive(Debug, PartialEq)]
struct Solver {
    values: Array2<i32>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
struct OutputStep {
    row: i32,
    col: i32,
    weight: usize,
    from_len: usize,
}

impl Solver {
    fn is_inside(&self, pos: (i32, i32)) -> bool {
        let d = self.values.dim();
        pos.0 >= 0 && pos.1 >= 0 && (pos.0 as usize) < d.0 && (pos.1 as usize) < d.1
    }

    // Return the next position in this direction
    fn next(&self, pos: (i32, i32), direction: Direction) -> Option<(i32, i32, usize)> {
        let delta = match direction {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        };
        let next = (pos.0 + delta.0, pos.1 + delta.1);
        if self.is_inside(next) {
            Some((
                next.0,
                next.1,
                *self
                    .values
                    .get((next.0 as usize, next.1 as usize))
                    .expect("inside") as usize,
            ))
        } else {
            None
        }
    }

    fn successors(&self, pos: &SolveLocation) -> Vec<(SolveLocation, usize)> {
        let mut result = Vec::with_capacity(2);

        for direction in [
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ] {
            if direction == pos.from_direction.invert() {
                // may not go back
                continue;
            }

            if pos.from_direction == direction && pos.from_len >= 3 {
                // may not go too deep
                continue;
            }

            let next = match self.next((pos.row, pos.col), direction) {
                None => continue,
                Some(v) => v,
            };

            let mut from_len = 1;
            if (pos.row == 0) && (pos.col == 0) {
                from_len = 2; // extra cost for start
            } else if direction == pos.from_direction {
                from_len = pos.from_len + 1;
            }

            // Allow moving foward
            let loc = SolveLocation {
                row: next.0,
                col: next.1,
                from_direction: direction,
                from_len,
            };

            trace!(
                "For [{:?} + {:?}]: {:?} weight {}",
                pos,
                direction,
                &loc,
                next.2
            );
            result.push((loc, next.2))
        }
        result
    }

    fn shortest_path(&self, pos: SolveLocation, goal: (usize, usize)) -> usize {
        let (target_row, target_col) = (goal.0 as i32, goal.1 as i32);

        info!("Shortest path compute...");

        // start with a particular location and try to reach the goal
        let result = dijkstra(
            &pos,
            |p| self.successors(p),
            |p| (p.row == target_row && p.col == target_col),
        );

        let solution = result.expect("Dijkstra finds a solution");

        info!("Shortest path:\n{:#?}", solution);

        let cost = solution
            .0
            .iter()
            .map(|l| {
                *self
                    .values
                    .get((l.row as usize, l.col as usize))
                    .expect("valid position")
            })
            .sum::<i32>() as usize;
        info!("Actual cost: {} vs {}", cost, solution.1);
        solution.1
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

    info!("Input:\n{:#?}", result);

    result
}

pub fn part1(input: &str) -> usize {
    let solver = Solver {
        values: parse_input(input),
    };

    let d = solver.values.dim();
    let goal = (d.0 - 1, d.1 - 1);

    solver.shortest_path(
        SolveLocation {
            row: 0,
            col: 0,
            from_direction: Direction::Up,
            from_len: 0,
        },
        goal,
    )
}

pub fn part2(_input: &str) -> usize {
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

    #[test_log::test]
    fn test_part1() {
        /*
                assert_eq!(
                    part1(
                        "
        2121
        9921
        9913
                "
                        .trim()
                    ),
                    11
                );
                */
        assert_eq!(part1(include_str!("../example.txt")), 102);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../example.txt")), 0);
    }
}
