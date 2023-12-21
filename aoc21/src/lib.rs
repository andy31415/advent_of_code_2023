use std::collections::HashSet;

type Position = (i32, i32); // row, col

struct Input {
    rows: usize,
    cols: usize,
    stones: HashSet<Position>,
    start: Position,
}

impl Input {
    fn directions(&self, p: Position) -> impl Iterator<Item = Position> + '_ {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(move |(r, c)| (p.0 + *r, p.1 + *c))
            .filter(|p| p.0 >= 0 && p.0 < self.rows as i32 && p.1 >= 0 && p.1 < self.cols as i32)
            .filter(|p| !self.stones.contains(p))
    }

    fn count_even(&self, steps: usize) -> usize {
        let mut seen = HashSet::new();
        let mut even = HashSet::new();

        let mut bfs = Vec::new();
        bfs.push(self.start);

        for step in 0..steps {
            // actual step index is step + 1
            let mut next_step = Vec::new();

            while let Some(p) = bfs.pop() {
                for ns in self.directions(p) {
                    if seen.contains(&ns) {
                        continue;
                    }
                    seen.insert(ns);
                    next_step.push(ns);
                    
                    if step % 2 == 1 {
                        even.insert(ns);
                    }
                }
            }

            bfs.append(&mut next_step);
        }

        even.len()
    }
}

fn parse_input(input: &str) -> Input {
    let mut rows = 0;
    let mut cols = None;
    let mut start = None;
    let mut stones = HashSet::new();

    for (row, l) in input.split("\n").enumerate().map(|(r, l)| (r as i32, l)) {
        match cols {
            Some(v) => assert!(l.len() == v),
            None => cols = Some(l.len()),
        }

        for (col, c) in l.chars().enumerate().map(|(c, l)| (c as i32, l)) {
            match c {
                '.' => (),
                '#' => {
                    stones.insert((row, col));
                }
                'S' => {
                    assert!(start.is_none());
                    start = Some((row, col));
                }
                _ => panic!("Invalid input: '{}' is unknown", c),
            }
        }

        rows += 1;
    }

    Input {
        rows,
        cols: cols.expect("valid input - has cols"),
        start: start.expect("valid input - has start"),
        stones,
    }
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);
    input.count_even(64)
}

pub fn part2(input: &str) -> usize {
    // TODO: implement
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steps() {
       let input = parse_input(include_str!("../example.txt"));
       
       assert_eq!(input.count_even(2), 4);
       assert_eq!(input.count_even(6), 16);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../example.txt")), 42);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../example.txt")), 0);
    }
}
