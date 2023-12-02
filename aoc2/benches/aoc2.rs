use aoc2::Game;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part2() {
    divan::black_box(include_str!("../input.txt"))
        .split('\n')
        .filter_map(Game::parse)
        .map(|g| g.min_bag().power())
        .sum::<u32>();
}
