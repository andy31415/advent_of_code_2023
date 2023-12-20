use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::space0,
    combinator::value,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult, Parser,
};
use nom_supreme::ParserExt;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Operation {
    Broadcast,
    Conjunction,
    FlipFlop,
}

#[derive(Debug, PartialEq, Clone)]
struct Module<'a> {
    name: &'a str,
    operation: Operation,
    targets: Vec<&'a str>,
}

#[derive(Debug, Clone)]
struct Input<'a> {
    broadast_targets: Vec<&'a str>,
    modules: HashMap<&'a str, Module<'a>>,
}

fn label(s: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphabetic()).parse(s)
}

fn module(i: &str) -> IResult<&str, Module> {
    separated_pair(
        alt((
            value((Operation::Broadcast, "broadcaster"), tag("broadcaster")),
            label
                .preceded_by(tag("&"))
                .map(|l| (Operation::Conjunction, l)),
            label
                .preceded_by(tag("%"))
                .map(|l| (Operation::FlipFlop, l)),
        )),
        tuple((space0, tag("->"), space0)),
        separated_list1(tuple((space0, tag(","), space0)), label),
    )
    .map(|((operation, name), targets)| Module {
        name,
        operation,
        targets,
    })
    .parse(i)
}

pub fn part1(input: &str) -> usize {
    // TODO: implement
    0
}

pub fn part2(input: &str) -> usize {
    // TODO: implement
    0
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("broadcaster -> a, b, c",
           Module{ name: "broadcaster", operation: Operation::Broadcast, targets: vec!["a", "b", "c"]})]
    #[case("%a -> b", Module{ name: "a", operation: Operation::FlipFlop, targets: vec!["b"]})]
    #[case("&inv -> a", Module{ name: "inv", operation: Operation::Conjunction, targets: vec!["a"]})]
    fn test_parse_module(#[case] txt: &str, #[case] value: Module) {
        let (r, m) = module(txt).expect("valid");
        assert_eq!(m, value);
        assert_eq!(r, "");
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../example.txt")), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../example.txt")), 0);
    }
}
