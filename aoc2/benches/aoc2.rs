use aoc2::{Bag, Game};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };
    divan::black_box(include_str!("../input.txt"))
        .split('\n')
        .filter_map(Game::parse)
        .filter(|g| g.possible(&bag))
        .map(|g| g.id)
        .sum::<u32>();
}

#[divan::bench]
fn part2() {
    divan::black_box(include_str!("../input.txt"))
        .split('\n')
        .filter_map(Game::parse)
        .map(|g| g.min_bag().power())
        .sum::<u32>();
}
