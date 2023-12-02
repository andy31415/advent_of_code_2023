use aoc::Game;

fn main() {
    println!("Testing");

    let id_sum: u32 = include_str!("../../input.txt")
        .split('\n')
        .filter_map(Game::parse)
        .map(|g| g.min_bag().power())
        .sum();

    println!("SUM of ID: {}", id_sum);
}
