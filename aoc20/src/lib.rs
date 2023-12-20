use std::collections::{HashMap, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{line_ending, space0},
    combinator::value,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult, Parser,
};
use nom_supreme::ParserExt;
use tracing::trace;

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
    broadcast_targets: Vec<&'a str>,
    modules: HashMap<&'a str, Module<'a>>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum PulseState {
    Low,
    High,
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
struct FlipFlopState {
    on: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct ConjunctionState<'a> {
    inputs: HashMap<&'a str, PulseState>,
}

#[derive(Debug, Clone, PartialEq)]
enum ModuleState<'a> {
    FlipFlop(FlipFlopState),
    Conjunction(ConjunctionState<'a>),
}

#[derive(Debug, Clone)]
struct Solver<'a> {
    input: Input<'a>,
    state: HashMap<&'a str, ModuleState<'a>>,
}

impl<'a> Solver<'a> {
    // Broadcasts a pulse and handles it. Returns the number of
    // pulses sent around
    fn pulse(&mut self) -> (usize, usize) {
        let mut instructions = VecDeque::new();

        let mut low_count = 0;
        let mut high_count = 0;

        low_count += 1; // pulse to broadcaster

        for v in self.input.broadcast_targets.iter() {
            instructions.push_back(("broadcast", *v, PulseState::Low));
        }

        while let Some((source, target, pulse)) = instructions.pop_front() {
            match pulse {
                PulseState::Low => low_count += 1,
                PulseState::High => high_count += 1,
            }
            instructions.append(&mut self.send_pulse(source, target, pulse));
        }
        (low_count, high_count)
    }

    fn send_pulse<'b, 'c>(
        &'b mut self,
        source: &'a str,
        target: &'a str,
        pulse: PulseState,
    ) -> VecDeque<(&'a str, &'a str, PulseState)> {
        trace!("PULSE: {}: {:?} to {}", source, pulse, target);
        let mut result = VecDeque::new();

        let state = match self.state.get_mut(target) {
            Some(s) => s,
            None => {
                // some sanity check ... eventually pulses must end
                // assert_eq!(target, "output");
                return result;
            }
        };

        match state {
            ModuleState::FlipFlop(f) => {
                if pulse == PulseState::Low {
                    f.on = !f.on;
                    let pulse_type = if f.on {
                        PulseState::High
                    } else {
                        PulseState::Low
                    };
                    for t in self
                        .input
                        .modules
                        .get(target)
                        .expect("valid target module")
                        .targets
                        .iter()
                    {
                        result.push_back((target, *t, pulse_type))
                    }
                }
            }
            ModuleState::Conjunction(c) => {
                *c.inputs.get_mut(source).expect("valid source") = pulse;

                let pulse_type = if c.inputs.values().all(|s| *s == PulseState::High) {
                    PulseState::Low
                } else {
                    PulseState::High
                };

                for t in self
                    .input
                    .modules
                    .get(target)
                    .expect("valid target module")
                    .targets
                    .iter()
                {
                    result.push_back((target, *t, pulse_type))
                }
            }
        }

        result
    }
}

impl<'a> From<Input<'a>> for Solver<'a> {
    fn from(input: Input<'a>) -> Self {
        let mut state = HashMap::new();

        for m in input.modules.values() {
            match m.operation {
                Operation::Conjunction => {
                    state.insert(
                        m.name,
                        ModuleState::Conjunction(ConjunctionState::default()),
                    );
                }
                Operation::FlipFlop => {
                    state.insert(m.name, ModuleState::FlipFlop(FlipFlopState::default()));
                }
                _ => {}
            }
        }

        // every conjunction has to remember inputs. Go through them again
        for m in input.modules.values() {
            // for every target of this module, if the module is a conjunction update its state
            for t in m.targets.iter() {
                if let Some(ModuleState::Conjunction(fs)) = state.get_mut(t) {
                    fs.inputs.insert(m.name, PulseState::Low);
                }
            }
        }

        Self { input, state }
    }
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

fn parse_input(s: &str) -> Input {
    let (r, mvec) = separated_list1(line_ending, module)
        .parse(s)
        .expect("valid input");
    assert_eq!(r, "");

    let mut broadcast_targets = None;
    let mut modules = HashMap::new();

    for m in mvec {
        if m.operation == Operation::Broadcast {
            broadcast_targets = Some(m.targets.clone());
        }
        modules.insert(m.name, m);
    }
    let broadcast_targets = broadcast_targets.expect("has broadcast");
    Input {
        broadcast_targets,
        modules,
    }
}

pub fn part1(input: &str) -> usize {
    let mut solver: Solver = parse_input(input).into();

    let mut low = 0;
    let mut high = 0;
    for _ in 0..1000 {
        let (l, h) = solver.pulse();
        trace!(
            "-----------------TOTAL: {}, {} ------------------------",
            l,
            h
        );
        low += l;
        high += h;
    }

    low * high
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

    #[test_log::test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../example.txt")), 32000000);
        assert_eq!(part1(include_str!("../example2.txt")), 11687500);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../example.txt")), 0);
    }
}
