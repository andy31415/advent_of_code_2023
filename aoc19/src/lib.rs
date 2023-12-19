use std::{collections::HashMap, fmt::Display};

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
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Debug, Clone, PartialEq, Copy)]
struct PartRange {
    x: (u64, u64), // NOT including the upper bound
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

impl Display for PartRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "PR[x: {}..{}, m: {}..{}, a: {}..{}, s: {}..{}]",
            self.x.0, self.x.1, self.m.0, self.m.1, self.a.0, self.a.1, self.s.0, self.s.1
        ))
    }
}

impl PartRange {
    fn variations(&self) -> usize {
        ((self.x.1 - self.x.0)
            * (self.m.1 - self.m.0)
            * (self.a.1 - self.a.0)
            * (self.s.1 - self.s.0)) as usize
    }
}

impl Part {
    fn value(&self, v: Variable) -> u64 {
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
    value: u64,
}

impl Condition {
    fn matches(&self, part: &Part) -> bool {
        let v = part.value(self.variable);

        match self.compare {
            Compare::GT => v > self.value,
            Compare::LT => v < self.value,
        }
    }

    /// Move the given range into accept/reject
    fn split_range(&self, r: (u64, u64)) -> (Option<(u64, u64)>, Option<(u64, u64)>) {
        if self.value < r.0 {
            // all values are larger than the target
            return match self.compare {
                Compare::GT => (Some(r), None), // expect larger, so accept
                Compare::LT => (None, Some(r)), // expect smaller, so reject
            };
        }
        if self.value >= r.1 {
            // all values are smaller than the target
            return match self.compare {
                Compare::GT => (None, Some(r)), // expect larger, so reject
                Compare::LT => (Some(r), None), // expect smaller, so accept
            };
        }

        // need to split the range, but also take into consideration
        // the edges
        match self.compare {
            Compare::GT => (
                Some((self.value + 1, r.1)), // accept all greater than
                Some((r.0, self.value + 1)), // reject all less or equal
            ),
            Compare::LT => (
                Some((r.0, self.value)), // accept all less
                Some((self.value, r.1)), // reject all larger
            ),
        }
    }

    /// Given an input range, split it into MATCHES vs NOT MATCHING
    fn split(&self, part: &PartRange) -> (Option<PartRange>, Option<PartRange>) {
        let (lx, rx) = if self.variable == Variable::X {
            self.split_range(part.x)
        } else {
            (Some(part.x), Some(part.x))
        };

        let (lm, rm) = if self.variable == Variable::M {
            self.split_range(part.m)
        } else {
            (Some(part.m), Some(part.m))
        };

        let (la, ra) = if self.variable == Variable::A {
            self.split_range(part.a)
        } else {
            (Some(part.a), Some(part.a))
        };

        let (ls, rs) = if self.variable == Variable::S {
            self.split_range(part.s)
        } else {
            (Some(part.s), Some(part.s))
        };

        (
            match (lx, lm, la, ls) {
                (Some(x), Some(m), Some(a), Some(s)) => Some(PartRange { x, m, a, s }),
                _ => None,
            },
            match (rx, rm, ra, rs) {
                (Some(x), Some(m), Some(a), Some(s)) => Some(PartRange { x, m, a, s }),
                _ => None,
            },
        )
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

    fn split(&self, part: &PartRange) -> (Option<PartRange>, Option<PartRange>) {
        match self.condition {
            Some(c) => c.split(part),
            None => (Some(*part), None),
        }
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

    fn split(&self, part: &PartRange) -> Vec<(&'a str, PartRange)> {
        let mut result = Vec::new();
        let mut remaining = *part;
        for rule in self.rules.iter() {
            let (a, r) = rule.split(&remaining);
            if let Some(r) = a {
                result.push(((rule.target), r));
            }
            remaining = match r {
                Some(r) => r,
                None => break,
            };
        }
        result
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

    fn all_accepted(&self, part: &PartRange) -> Vec<PartRange> {
        // go through all rules until nothing is left
        let mut result = Vec::new();
        let mut tasks = Vec::new();

        tasks.push((self.start, *part));

        while let Some(task) = tasks.pop() {
            trace!("Next task: {} with {}", task.0.name, task.1);

            for (target, r) in task.0.split(&task.1) {
                trace!("  Split {} -> {}", r, target);
                match target.try_into() {
                    Ok(FinalState::Accept) => result.push(r),
                    Ok(FinalState::Reject) => (),

                    // not a final state, keep going
                    Err(_) => tasks.push((self.workflows.get(target).expect("valid target"), r)),
                }
            }
        }

        result
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
        nom::character::complete::u64,
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
        nom::character::complete::u64
            .preceded_by(tag("x="))
            .terminated(tag(",")),
        nom::character::complete::u64
            .preceded_by(tag("m="))
            .terminated(tag(",")),
        nom::character::complete::u64
            .preceded_by(tag("a="))
            .terminated(tag(",")),
        nom::character::complete::u64.preceded_by(tag("s=")),
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

pub fn part2(s: &str) -> usize {
    let data = input(s);
    let solver: Solver = (&data).into();

    let meta_part = PartRange {
        x: (1, 4001),
        m: (1, 4001),
        a: (1, 4001),
        s: (1, 4001),
    };

    solver
        .all_accepted(&meta_part)
        .iter()
        .map(|p| p.variations())
        .sum()
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

    #[test_log::test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../example.txt")), 167409079868000);
    }
}
