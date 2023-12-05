use aoc4::{part_1_add_points, part_2_sum_cards};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part_1_add_points(divan::black_box(include_str!("../input.txt")));
}

#[divan::bench]
fn part2() {
    part_2_sum_cards(divan::black_box(include_str!("../input.txt")));
}
