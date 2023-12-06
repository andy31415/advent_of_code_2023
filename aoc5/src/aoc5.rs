use aoc5::{part_1_min, part_2_min};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let s1 = part_1_min(include_str!("../input.txt"));
    println!("Part 1: {}", s1);

    let s2 = part_2_min(include_str!("../input.txt"));
    println!("Part 2: {}", s2);
}
