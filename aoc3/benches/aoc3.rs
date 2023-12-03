use aoc3::part_1_sum_parts;

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
    // println!("TODO: bench part 2");
}
