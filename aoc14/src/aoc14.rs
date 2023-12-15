use aoc14::{part1, part2};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let s1 = part1(include_str!("../input.txt"));
    println!("Part 1: {}", s1);

    let s2 = part2(include_str!("../input.txt"), 1000000000);
    println!("Part 2: {}", s2);
}
