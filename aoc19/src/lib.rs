use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::line_ending,
    combinator::value,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult, Parser,
};
use nom_supreme::ParserExt;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Variable {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Compare {
    GT,
    LT,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Condition {
    variable: Variable,
    compare: Compare,
    value: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct Rule<'a> {
    condition: Option<Condition>,
    target: &'a str,
}

#[derive(Debug, Clone, PartialEq)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
struct Input<'a> {
    workflows: Vec<Workflow<'a>>,
    parts: Vec<Part>,
}

fn condition(s: &str) -> IResult<&str, Condition> {
    tuple((
        alt((
            value(Variable::X, tag("x")),
            value(Variable::M, tag("m")),
            value(Variable::A, tag("a")),
            value(Variable::S, tag("s")),
        )),
        alt((value(Compare::LT, tag("<")), value(Compare::GT, tag(">")))),
        nom::character::complete::i32,
    ))
    .map(|(variable, compare, value)| Condition {
        variable,
        compare,
        value,
    })
    .parse(s)
}

fn label(s: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphabetic()).parse(s)
}

fn rule(s: &str) -> IResult<&str, Rule> {
    tuple((condition.terminated(tag(":")).opt(), label))
        .map(|(condition, target)| Rule { condition, target })
        .parse(s)
}

fn workflow(s: &str) -> IResult<&str, Workflow> {
    tuple((
        label,
        separated_list1(tag(","), rule)
            .preceded_by(tag("{"))
            .terminated(tag("}")),
    ))
    .map(|(name, rules)| Workflow { name, rules })
    .parse(s)
}

fn input(s: &str) -> Input {
    let (r, i) = separated_pair(
        separated_list1(line_ending, workflow),
        tuple((line_ending, line_ending)),
        separated_list1(line_ending, part),
    )
    .map(|(workflows, parts)| Input { workflows, parts })
    .parse(s)
    .expect("valid input");

    assert_eq!(r, "");
    i
}

fn part(s: &str) -> IResult<&str, Part> {
    tuple((
        nom::character::complete::i32
            .preceded_by(tag("x="))
            .terminated(tag(",")),
        nom::character::complete::i32
            .preceded_by(tag("m="))
            .terminated(tag(",")),
        nom::character::complete::i32
            .preceded_by(tag("a="))
            .terminated(tag(",")),
        nom::character::complete::i32.preceded_by(tag("s=")),
    ))
    .preceded_by(tag("{"))
    .terminated(tag("}"))
    .map(|(x, m, a, s)| Part { x, m, a, s })
    .parse(s)
}

pub fn part1(s: &str) -> usize {
    let data = input(s);

    eprintln!("{:?}", data);
    // TODO: implement
    0
}

pub fn part2(_s: &str) -> usize {
    // TODO: implement
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("A", Rule { target: "A", condition: None})]
    #[case("R", Rule { target: "R", condition: None})]
    #[case("rfg", Rule { target: "rfg", condition: None})]
    #[case("gd", Rule { target: "gd", condition: None})]
    #[case("a<2006:gd", Rule { target: "gd", condition: Some(Condition{ variable: Variable::A, compare: Compare::LT, value: 2006})})]
    #[case("s>3448:pv", Rule { target: "pv", condition: Some(Condition{ variable: Variable::S, compare: Compare::GT, value: 3448})})]
    fn parse_rule(#[case] s: &str, #[case] expected: Rule) {
        assert_eq!(rule(s).expect("valid").1, expected);
    }

    #[rstest]
    #[case("x<9999", Condition{ variable: Variable::X, compare: Compare::LT, value: 9999})]
    #[case("m>1234", Condition{ variable: Variable::M, compare: Compare::GT, value: 1234})]
    #[case("a<2006", Condition{ variable: Variable::A, compare: Compare::LT, value: 2006})]
    #[case("s>3448", Condition{ variable: Variable::S, compare: Compare::GT, value: 3448})]
    fn parse_condition(#[case] s: &str, #[case] expected: Condition) {
        assert_eq!(condition(s).expect("valid").1, expected);
    }

    #[test]
    fn parse_part() {
        assert_eq!(
            part("{x=787,m=2655,a=1222,s=2876}").expect("valid").1,
            Part {
                x: 787,
                m: 2655,
                a: 1222,
                s: 2876
            }
        );
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
