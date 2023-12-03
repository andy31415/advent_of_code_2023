use aoc3::part_1_sum_parts;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let s = part_1_sum_parts(include_str!("../input.txt"));
    println!("Part 1: {}", s);

    println!("Part 2:");
}
