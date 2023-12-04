use aoc3::{
    alternate_part_1_sum_parts, alternate_part_2_sum_gear_ratios, part_1_sum_parts,
    part_2_sum_gear_ratios,
};

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

#[divan::bench]
fn alternate_part1() {
    alternate_part_1_sum_parts(divan::black_box(include_str!("../input.txt")));
}

#[divan::bench]
fn alternate_part2() {
    alternate_part_2_sum_gear_ratios(divan::black_box(include_str!("../input.txt")));
}
