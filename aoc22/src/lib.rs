use std::{collections::HashMap, fmt::Debug};

use glam::IVec3;
use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult, Parser,
};
use nom_supreme::ParserExt;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Brick {
    start: IVec3,
    end: IVec3,
}

impl Debug for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // compact debug
        f.write_fmt(format_args!(
            "Brick[s: {}, {}, {} e: {}, {}, {}]",
            self.start.x, self.start.y, self.start.z, self.end.x, self.end.y, self.end.z
        ))
    }
}

impl Brick {
    fn bottom_z(&self) -> i32 {
        self.start.z.min(self.end.z)
    }

    fn top_z(&self) -> i32 {
        self.start.z.max(self.end.z)
    }

    fn drop_z(&mut self, cnt: i32) {
        self.start.z -= cnt;
        self.end.z -= cnt;
    }

    fn intesects_xy(&self, other: &Brick) -> bool {
        if (self.end.x < other.start.x) || (other.end.x < self.start.x) {
            return false;
        }
        if (self.end.y < other.start.y) || (other.end.y < self.start.y) {
            return false;
        }

        return true;
    }
}

struct Building {
    bricks: Vec<Brick>,
    by_top_z: HashMap<i32, Vec<usize>>, // z-index to brick index
}

const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn idx_to_human(idx: usize) -> String {
    let extra_idx = idx / LETTERS.len();
    let idx = idx % LETTERS.len();

    if extra_idx == 0 {
        return LETTERS[idx..=idx].into();
    }

    format!("{}{}", &LETTERS[idx..=idx], extra_idx)
}

impl Debug for Building {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Building {\n  bricks: [\n")?;

        for (idx, b) in self.bricks.iter().enumerate() {
            f.write_fmt(format_args!("    {:?} // {}\n", b, idx_to_human(idx)))?;
        }
        f.write_str("  ]\n")?;

        f.write_str("  by_top_z: [\n")?;

        let mut keys: Vec<_> = self.by_top_z.keys().collect();
        keys.sort();
        keys.reverse();

        for idx in keys {
            f.write_fmt(format_args!("    {}: [ ", idx))?;

            for (c, v) in self.by_top_z.get(idx).expect("is a key").iter().enumerate() {
                if c != 0 {
                    f.write_str(", ")?;
                }
                f.write_fmt(format_args!("{}/{}", v, idx_to_human(*v)))?;
            }

            f.write_str(" ]\n")?;
        }

        f.write_str("  ]\n")?;

        //f.debug_struct("Building").field("bricks", &self.bricks).field("by_top_z", &self.by_top_z).finish()
        //
        f.write_str("}")
    }
}

impl Building {
    fn new(mut input: Vec<Brick>) -> Self {
        let mut result = Building {
            bricks: Vec::new(),
            by_top_z: HashMap::new(),
        };

        // make sure lower z items drop first
        input.sort_by(|a, b| a.bottom_z().cmp(&b.bottom_z()));

        for brick in input {
            result.drop_brick(brick);
        }
        result
    }

    fn brick_with_index(&self, idx: usize) -> &Brick {
        self.bricks.get(idx).expect("Valid brick index")
    }

    fn drop_brick(&mut self, mut b: Brick) {
        'drop_loop: while b.bottom_z() > 1 {
            // check if we can drop one
            if let Some(v) = self.by_top_z.get(&(b.bottom_z() - 1)) {
                for other in v.iter().map(|idx| self.brick_with_index(*idx)) {
                    if b.intesects_xy(other) {
                        break 'drop_loop;
                    }
                }
            }

            b.drop_z(1);
        }

        let brick_idx = self.bricks.len();
        self.bricks.push(b);

        if let Some(v) = self.by_top_z.get_mut(&b.top_z()) {
            v.push(brick_idx);
        } else {
            self.by_top_z.insert(b.top_z(), vec![brick_idx]);
        }
    }
}

fn vec3d(s: &str) -> IResult<&str, IVec3> {
    tuple((
        nom::character::complete::i32.terminated(tag(",")),
        nom::character::complete::i32.terminated(tag(",")),
        nom::character::complete::i32,
    ))
    .map(|(x, y, z)| IVec3::new(x, y, z))
    .parse(s)
}

fn line(s: &str) -> IResult<&str, (IVec3, IVec3)> {
    separated_pair(vec3d, tag("~"), vec3d).parse(s)
}

fn parse_input(s: &str) -> Vec<Brick> {
    let (r, i) = separated_list1(line_ending, line.map(|(start, end)| Brick { start, end }))
        .parse(s)
        .expect("Valid input");
    assert_eq!(r, "");
    i
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);
    let b = Building::new(input);

    dbg!(&b);

    0
}

pub fn part2(_input: &str) -> usize {
    // TODO: implement
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brick_intersect() {
        assert!(!Brick {
            start: IVec3::new(0, 0, 4),
            end: IVec3::new(0, 2, 4)
        }
        .intesects_xy(&Brick {
            start: IVec3::new(2, 0, 5),
            end: IVec3::new(2, 2, 5)
        }));
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
