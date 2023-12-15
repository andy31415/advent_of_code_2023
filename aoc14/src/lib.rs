#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Copy, Clone)]
enum Item {
    Free,
    Movable,
    Immovable,
}

impl From<char> for Item {
    fn from(value: char) -> Self {
        match value {
            '.' => Item::Free,
            'O' => Item::Movable,
            '#' => Item::Immovable,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Map {
    data: Vec<Vec<Item>>,
}

impl Map {
    fn swap(&mut self, a: (usize, usize), b: (usize, usize)) {
        let data_a = *self.data.get(a.0).and_then(|v| v.get(a.1)).expect("valid a");
        let data_b = *self.data.get(b.0).and_then(|v| v.get(b.1)).expect("valid b");

        *self.data.get_mut(a.0).and_then(|v| v.get_mut(a.1)).expect("valid b") = data_b;
        *self.data.get_mut(b.0).and_then(|v| v.get_mut(b.1)).expect("valid a") = data_a;

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
    fn test_swap() {
        let mut map = parse_map("#.O\nOOO\n..#");
        map.swap((0,0), (2,1));
        
        assert_eq!(
            map,
        Map{
            data: vec![
                vec![Item::Free, Item::Free, Item::Movable],
                vec![Item::Movable, Item::Movable, Item::Movable],
                vec![Item::Free, Item::Immovable, Item::Immovable],
            ],
        });
    }

    #[test]
    fn test_map_parse() {
        assert_eq!(parse_map("#.O\nOOO\n..#"),
        Map{
            data: vec![
                vec![Item::Immovable, Item::Free, Item::Movable],
                vec![Item::Movable, Item::Movable, Item::Movable],
                vec![Item::Free, Item::Free, Item::Immovable],
            ],
        });
    }
}
