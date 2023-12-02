use aoc::Game;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    println!("Testing");

    let id_sum: u32 = include_str!("../../input.txt")
        .split('\n')
        .filter_map(Game::parse)
        .map(|g| g.min_bag().power())
        .sum();

    println!("SUM of ID: {}", id_sum);
}
