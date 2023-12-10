use aoc10::{part1, part2};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::prelude::*;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let stdout_log = tracing_subscriber::fmt::layer().pretty();

    tracing_subscriber::registry()
        .with(
            stdout_log.with_filter(LevelFilter::WARN),
        )
        .init();

    let s1 = part1(include_str!("../input.txt"));
    println!("Part 1: {}", s1);

    let s2 = part2(include_str!("../input.txt"));
    println!("Part 2: {}", s2);
}
