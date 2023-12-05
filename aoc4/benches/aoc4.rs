use aoc4::part_1_add_points;

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
    // println!("TODO: bench part 2");
}
