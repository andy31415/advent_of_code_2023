use aoc3::{part_1_sum_parts, part_2_sum_gear_ratios};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part_1_sum_parts(divan::black_box(include_str!("../input.txt")));
}

#[divan::bench]
fn part2() {
    part_2_sum_gear_ratios(divan::black_box(include_str!("../input.txt")));
}
