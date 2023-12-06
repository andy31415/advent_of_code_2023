use aoc5::{part_1_min, part_2_min};
use divan::black_box;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part_1_min(black_box(include_str!("../input.txt")));
}

#[divan::bench]
fn part2() {
    part_2_min(black_box(include_str!("../input.txt")));
}
