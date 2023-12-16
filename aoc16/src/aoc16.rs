#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let s1 = aoc16::part1(include_str!("../input.txt"));
    println!("Part 1: {}", s1);

    let s2 = aoc16::part2(include_str!("../input.txt"));
    println!("Part 2: {}", s2);
}
