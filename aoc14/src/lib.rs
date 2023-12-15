#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Cell {
    Free,
    Movable,
    Immovable,
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Map {
    data: Vec<Vec<Cell>>,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Cell::Free,
            'O' => Cell::Movable,
            '#' => Cell::Immovable,
            _ => unreachable!(),
        }
    }
}

fn parse_map(input: &str) -> Map {
    Map {
        data: input
            .split('\n')
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_parse() {
        assert_eq!(parse_map("#.O\nOOO\n..#"),
        Map{
            data: vec![
                vec![Cell::Immovable, Cell::Free, Cell::Movable],
                vec![Cell::Movable, Cell::Movable, Cell::Movable],
                vec![Cell::Free, Cell::Free, Cell::Immovable],
            ],
        });
    }
}
