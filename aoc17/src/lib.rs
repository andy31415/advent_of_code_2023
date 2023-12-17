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

/// A location in a solution
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
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
        let mut result = Vec::with_capacity(9);

        for direction in [
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ] {
            if pos.deny == direction {
                continue;
            }

            let choices = [0, 1, 2]
                .iter()
                .scan(Some((pos.row, pos.col, 0)), |state, _| {
                    let state = match state {
                        Some(v) => v,
                        None => return None,
                    };
                    let next = self
                        .next((state.0, state.1), direction)
                        .map(|(r, c, s)| (r, c, state.2 + s));
                    if let Some(v) = next {
                        *state = v;
                    }
                    next
                })
                .collect::<Vec<_>>();

            trace!("Choices [{:?} + {:?}]: {:?}", pos, direction, &choices);

            for c in choices {
                result.push((
                    SolveLocation {
                        deny: direction,
                        row: c.0,
                        col: c.1,
                    },
                    c.2,
                ))
            }
        }

        // TODO: grab weight + solve
        //
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

    let d = solver.values.dim();
    let goal = (d.0 - 1, d.1 - 1);

    let a = solver.shortest_path(
        SolveLocation {
            deny: Direction::Up,
            row: -1,
            col: 0,
        },
        goal,
    );
    let b = solver.shortest_path(
        SolveLocation {
            deny: Direction::Left,
            row: 0,
            col: -1,
        },
        goal,
    );

    *[a, b].iter().min().expect("have values")
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
        assert_eq!(part1(include_str!("../example.txt")), 102);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../example.txt")), 0);
    }
}
