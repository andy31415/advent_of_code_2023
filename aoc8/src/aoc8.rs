use aoc8::{part1_steps, part2_steps};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let s1 = part1_steps(include_str!("../input.txt"));
    println!("Part 1: {}", s1);

    let s2 = part2_steps(include_str!("../input.txt"));
    println!("Part 2: {}", s2);
}
