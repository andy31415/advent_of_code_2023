use std::collections::HashMap;

use tracing::{trace, info};

fn update_hash(current: u8, c: char) -> u8 {
    let x = current as usize;

    trace!("Hashing {} with '{}'", current, c);
    let x = x + c as usize;
    trace!("    With ASCII: {}", x);
    let x = x * 17;
    trace!("    Times 17: {}", x);

    (x & 0xFF) as u8
}

fn hash_string(s: &str) -> u8 {
    s.chars().fold(0, |acc, c| update_hash(acc, c))
}

pub fn part1(s: &str) -> usize {
    s.split('\n')
        .map(|l| l.split(','))
        .flatten()
        .fold(0, |acc, s| acc + hash_string(s) as usize)
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
enum Operation {
    Add(i32),
    Remove,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct Action<'a> {
    operation: Operation,
    label: &'a str,
}

impl<'a> From<&'a str> for Action<'a> {
    fn from(value: &'a str) -> Self {
        if let Some(pos) = value.find('=') {
            let (label, lens) = value.split_at(pos);
            return Self {
                label,
                operation: Operation::Add(lens[1..].parse().unwrap()),
            };
        }
        assert_eq!(value.chars().last().unwrap(), '-');

        return Self {
            operation: Operation::Remove,
            label: &value[0..(value.len() - 1)],
        };
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct Lens<'a> {
    label: &'a str,
    focus: i32,
}

#[derive(Debug)]
struct Mapping<'a> {
    map: HashMap<u8, Vec<Lens<'a>>>,
}

impl<'a> Mapping<'a> {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn perform(&mut self, action: &Action<'a>) {
        let h = hash_string(action.label);
        let b = self.map.get_mut(&h);

        match b {
            None => {
                if let Operation::Add(focus) = action.operation {
                    self.map.insert(
                        h,
                        vec![Lens {
                            label: action.label,
                            focus,
                        }],
                    );
                }
                // if not add, nothing to do as label does not exist
            }
            Some(v) => {
                match action.operation {
                    Operation::Add(focus) => {
                        // Choices: update existing item or append
                        match v.iter_mut().find(|lens| lens.label == action.label) {
                            Some(item) => item.focus = focus,
                            None => v.push(Lens {
                                label: action.label,
                                focus,
                            }),
                        }
                    }
                    Operation::Remove => v.retain(|lens| lens.label != action.label),
                }
            }
        }
    }

    fn total_focusing_power(&self) -> usize {
        let mut total = 0;
        for idx in 0..256 {
            let key = idx as u8;
            if let Some(v) = self.map.get(&key) {
                let multiplier = (idx + 1) as usize;
                total += multiplier
                    * v.iter()
                        .enumerate()
                        .fold(0, |acc, (idx, v)| acc + ((idx + 1) * v.focus as usize))
            }
        }
        total
    }
}

pub fn part2(s: &str) -> usize {
    let mut m = Mapping::new();
    for action in s
        .split('\n')
        .map(|l| l.split(','))
        .flatten()
        .map(|s| s.into())
    {
        m.perform(&action);
        info!("After {:?}: {:?}", &action, &m);
    }

    m.total_focusing_power()
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test_log::test]
    fn test_part2() {
        assert_eq!(
            part2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            145
        );

    }
    #[test]
    fn test_into_action() {
        assert_eq!(
            Action {
                operation: Operation::Add(1),
                label: "rn"
            },
            "rn=1".into()
        );
        assert_eq!(
            Action {
                operation: Operation::Remove,
                label: "cm"
            },
            "cm-".into()
        );
        assert_eq!(
            Action {
                operation: Operation::Add(3),
                label: "qp"
            },
            "qp=3".into()
        );
        assert_eq!(
            Action {
                operation: Operation::Remove,
                label: "pc"
            },
            "pc-".into()
        );
        assert_eq!(
            Action {
                operation: Operation::Add(7),
                label: "ot"
            },
            "ot=7".into()
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            1320
        );
    }

    #[test_log::test]
    fn test_hash() {
        assert_eq!(update_hash(0, 'H'), 200);
        assert_eq!(update_hash(200, 'A'), 153);
        assert_eq!(update_hash(153, 'S'), 172);
        assert_eq!(update_hash(172, 'H'), 52);
    }

    #[test]
    fn test_hash_string() {
        assert_eq!(hash_string("HASH"), 52);
        assert_eq!(hash_string("rn=1"), 30);
        assert_eq!(hash_string("cm-"), 253);
    }
}
