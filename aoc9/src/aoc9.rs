use aoc9::part1;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let s1 = part1(include_str!("../input.txt"));
    println!("Part 1: {}", s1);

    println!("Part 2:");
}
