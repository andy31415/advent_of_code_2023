use aoc2::{Bag, Game};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };
    let id_sum: u32 = include_str!("../input.txt")
        .split('\n')
        .filter_map(Game::parse)
        .filter(|g| g.possible(&bag))
        .map(|g| g.id)
        .sum();

    println!("SUM of ID: {}", id_sum);

    let power: u32 = include_str!("../input.txt")
        .split('\n')
        .filter_map(Game::parse)
        .map(|g| g.min_bag().power())
        .sum();

    println!("Power: {}", power);
}
