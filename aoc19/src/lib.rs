use std::collections::HashMap;

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
use tracing::{info, trace};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Variable {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Part {
    fn value(&self, v: Variable) -> i32 {
        match v {
            Variable::X => self.x,
            Variable::M => self.m,
            Variable::A => self.a,
            Variable::S => self.s,
        }
    }

    fn rating(&self) -> usize {
        (self.x + self.m + self.a + self.s) as usize
    }
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

impl Condition {
    fn matches(&self, part: &Part) -> bool {
        let v = part.value(self.variable);

        match self.compare {
            Compare::GT => v > self.value,
            Compare::LT => v < self.value,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Rule<'a> {
    condition: Option<Condition>,
    target: &'a str,
}

impl<'a> Rule<'a> {
    fn matches(&self, part: &Part) -> bool {
        self.condition.map(|c| c.matches(part)).unwrap_or(true)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

impl<'a> Workflow<'a> {
    fn next(&self, part: &Part) -> &'a str {
        for r in self.rules.iter() {
            if r.matches(part) {
                return r.target;
            }
        }
        panic!("Could not match {:?} in {:?}", part, self);
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Input<'a> {
    workflows: Vec<Workflow<'a>>,
    parts: Vec<Part>,
}

#[derive(Debug, Clone, PartialEq)]
struct Solver<'a> {
    start: &'a Workflow<'a>,
    workflows: HashMap<&'a str, &'a Workflow<'a>>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum FinalState {
    Accept,
    Reject,
}

impl TryFrom<&str> for FinalState {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "A" {
            return Ok(FinalState::Accept);
        }
        if value == "R" {
            return Ok(FinalState::Reject);
        }
        Err(())
    }
}

impl<'a> Solver<'a> {
    fn process(&self, part: &Part) -> FinalState {
        let mut flow = self.start;
        loop {
            trace!("{:?} -> flow {}", part, flow.name);
            let target = flow.next(part);

            flow = match target.try_into() {
                Ok(state) => return state,
                Err(_) => self.workflows.get(target).expect("valid target"),
            }
        }
    }
}

impl<'a> From<&'a Input<'a>> for Solver<'a> {
    fn from(value: &'a Input<'a>) -> Self {
        let mut workflows = HashMap::new();
        for w in value.workflows.iter() {
            workflows.insert(w.name, w);
        }
        let start = workflows.get("in").expect("has in workflow");

        Self { start, workflows }
    }
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
    let solver: Solver = (&data).into();

    let mut total = 0;

    for p in data.parts.iter() {
        if solver.process(p) == FinalState::Accept {
            info!("Accepted: {:?}", p);
            total += p.rating()
        } else {
            info!("Rejected: {:?}", p);
        }
    }

    total
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

    #[test_log::test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../example.txt")), 19114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../example.txt")), 0);
    }
}
