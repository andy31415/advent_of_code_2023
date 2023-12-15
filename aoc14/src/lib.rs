#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Cell {
    Free,
    Movable,
    Immovable,
}



#[cfg(test)]
mod tests {

    #[test]
    fn example() {
        assert_eq!(2 + 2, 4);
    }
}
