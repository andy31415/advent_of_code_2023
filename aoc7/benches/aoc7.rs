use aoc7::{part2_score, part1_score};
use divan::black_box;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1_score(black_box(include_str!("../input.txt")));
}

#[divan::bench]
fn part2() {
    part2_score(black_box(include_str!("../input.txt")));
}
