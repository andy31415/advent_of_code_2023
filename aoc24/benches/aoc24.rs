use divan::black_box;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    aoc24::part1(
        black_box(include_str!("../input.txt")),
        (200000000000000_f32, 400000000000000_f32),
    );
}

#[divan::bench]
fn part2() {
    aoc24::part2(black_box(include_str!("../input.txt")));
}
