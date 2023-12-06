use aoc5::part_1_min;
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
    // println!("TODO: bench part 2");
}
