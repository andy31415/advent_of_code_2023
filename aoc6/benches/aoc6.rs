use aoc6::{part_1, part_2};
use divan::black_box;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part_1(black_box(include_str!("../input.txt")));
}

#[divan::bench]
fn part2() {
    part_2(black_box(include_str!("../input.txt")));
}
