use std::fmt::Debug;

use glam::{Mat2, Vec2, Vec3};
use tracing::{info, instrument, trace};

#[derive(PartialEq, Copy, Clone)]
struct Hailstone {
    start: Vec3,
    direction: Vec3,
}

impl Debug for Hailstone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "HS[s: {:3},{:3},{:3} d:{:3},{:3},{:3}]",
            self.start.x,
            self.start.y,
            self.start.z,
            self.direction.x,
            self.direction.y,
            self.direction.z,
        ))
    }
}

mod parse {
    use glam::Vec3;
    use nom::{
        bytes::complete::tag,
        character::complete::{line_ending, space0},
        multi::separated_list1,
        sequence::{separated_pair, tuple},
        IResult, Parser,
    };
    use nom_supreme::ParserExt;

    use crate::Hailstone;

    fn vector(input: &str) -> IResult<&str, Vec3> {
        tuple((
            nom::character::complete::i64,
            nom::character::complete::i64.preceded_by(tuple((space0, tag(","), space0))),
            nom::character::complete::i64.preceded_by(tuple((space0, tag(","), space0))),
        ))
        .map(|(x, y, z)| Vec3::new(x as f32, y as f32, z as f32))
        .parse(input)
    }

    pub fn hailstone(input: &str) -> IResult<&str, Hailstone> {
        separated_pair(vector, tuple((space0, tag("@"), space0)), vector)
            .map(|(start, direction)| Hailstone { start, direction })
            .parse(input)
    }

    pub fn input(s: &str) -> Vec<Hailstone> {
        let (rest, result) = separated_list1(line_ending, hailstone)
            .parse(s)
            .expect("valid input");
        assert_eq!(rest, "");

        result
    }
}

impl Hailstone {
    #[instrument(skip_all)]
    fn intersect_2d(&self, other: &Hailstone) -> Option<Vec2> {
        // Look at 2d only
        let s1 = Vec2::new(self.start.x, self.start.y);
        let d1 = Vec2::new(self.direction.x, self.direction.y);

        let s2 = Vec2::new(other.start.x, other.start.y);
        let d2 = Vec2::new(other.direction.x, other.direction.y);

        let m = Mat2::from_cols(d1, -d2);

        if m.determinant() == 0.0 {
            return None;
        }
        let t = m.inverse() * (s2 - s1);

        if t.x < 0.0 || t.y < 0.0 {
            // interesect in the past
            return None;
        }

        // intersection. Both should be equal:
        //  t.x*d1 + s1
        //  t.y*d2 + s2
        Some(t.x * d1 + s1)
    }
}

pub fn part1(input: &str, range: (f32, f32)) -> usize {
    let stones = parse::input(input);

    info!("Stones: {}", stones.len());

    let mut cnt = 0;

    for (idx, a) in stones.iter().enumerate() {
        for b in stones.iter().skip(idx + 1) {
            if let Some(i) = a.intersect_2d(b) {
                if i.x >= range.0 && i.x <= range.1 && i.y >= range.0 && i.y <= range.1 {
                    cnt += 1;
                }
            }
        }
    }

    cnt
}

pub fn part2(input: &str) -> usize {
    // TODO: implement
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_intersect_2d() {
        let a = parse::hailstone("18, 19, 22 @ -1, -1, -2")
            .expect("valid")
            .1;
        let b = parse::hailstone("12, 31, 28 @ -1, -2, -1")
            .expect("valid")
            .1;

        assert_eq!(
            a.intersect_2d(&b).expect("intersection"),
            Vec2::new(-6.0, -5.0),
        );
        assert_eq!(
            b.intersect_2d(&a).expect("intersection"),
            Vec2::new(-6.0, -5.0),
        );
    }

    #[test_log::test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../example.txt"), (7_f32, 27_f32)), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../example.txt")), 0);
    }
}
