use ndarray::{Array, Array2};
use pathfinding::directed::dijkstra::dijkstra;
use tracing::{info, trace};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
enum Allow {
    Any,
    LeftRight,
    UpDown,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
struct Location {
    row: usize,
    col: usize,
    allow: Allow,
}

impl Location {
    fn position(&self) -> (usize, usize) {
        (self.row, self.col)
    }
    
    
    // Try to move current location
    fn constrained_move(&self, delta: (i32, i32), max: (usize, usize)) -> Option<Location> {
        let target_row = delta.0 + (self.row as i32);
        let target_col = delta.1 + (self.col as i32);

        let allow = match self.allow {
            Allow::Any if delta.0 == 0 => Allow::UpDown,
            Allow::Any if delta.1 == 0 => Allow::LeftRight,
            Allow::UpDown if delta.1 == 0 => Allow::LeftRight,
            Allow::LeftRight if delta.0 == 0 => Allow::UpDown,
            _ => return None,
        };
        

        (target_row >= 0
            && target_col >= 0
            && (target_row as usize) < max.0
            && (target_col as usize) < max.1)
            .then(|| Location {
                row: target_row as usize,
                col: target_col as usize,
                allow,
            })
    }
}

#[derive(Debug, PartialEq)]
struct Solver {
    values: Array2<i32>,
    min_len: usize,
    max_len: usize,
}

impl Solver {
    
    /// Computes the weight between pos and other,
    /// NOT including pos weight, but INCLUDING other weight
    fn weight(&self, pos: &Location, mut other: Location) -> usize {
        assert!(pos.row == other.row || pos.col == other.col);
        let mut total = 0;
        
        while other.position() != pos.position() {
            total += *self.values.get(other.position()).expect("valid position in map") as usize;
            if pos.row == other.row {
                if pos.col > other.col {
                    other.col += 1;
                } else {
                    other.col -= 1;
                    
                }
            } else if pos.col == other.col {
                if pos.row > other.row {
                    other.row += 1;
                } else {
                    other.row -= 1;
                    
                }
            } else {
                unreachable!();
            }

        }

        total
    }
    
    // Return available positions from the given location
    //
    // retunrs the weight INCLUDING the end, but NOT including the start
    fn successors(&self, pos: &Location) -> Vec<(Location, usize)> {
        let edge = self.values.dim();
        let deltas = (self.min_len..=self.max_len)
            .flat_map(|v| {
                [
                    (0, -(v as i32)),
                    (0, v as i32),
                    (-(v as i32), 0),
                    (v as i32, 0),
                ]
            })
            .filter_map(|c| pos.constrained_move(c, edge))
            .map(|p|{(p, self.weight(pos, p))})
            .collect();
 
        trace!("Deltas from {:?} -> {:?}", pos, deltas);

        deltas
    }

    fn shortest_path_to_end(&self, pos: Location) -> usize {
        let d = self.values.dim();
        let (target_row, target_col) = (d.0 - 1, d.1 - 1);

        // start with a particular location and try to reach the goal
        let result = dijkstra(
            &pos,
            |p| self.successors(p),
            |p| (p.row == target_row && p.col == target_col),
        );

        let solution = result.expect("Dijkstra finds a solution");

        info!("Shortest path:\n{:#?}", solution);

        // TODO: final cost is NOT expected

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
        min_len: 1,
        max_len: 3,
    };

    solver.shortest_path_to_end(Location {
        row: 0,
        col: 0,
        allow: Allow::Any,
    })
}

pub fn part2(input: &str) -> usize {
    let solver = Solver {
        values: parse_input(input),
        min_len: 4,
        max_len: 10,
    };

    solver.shortest_path_to_end(Location {
        row: 0,
        col: 0,
        allow: Allow::Any,
    })
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

    #[test_log::test]
    fn test_part2() {
        assert_eq!(
            part2(
                "
111111111111
999999999991
999999999991
999999999991
999999999991
        "
                .trim()
            ),
            71
        );
        assert_eq!(part2(include_str!("../example.txt")), 94);
    }
}
