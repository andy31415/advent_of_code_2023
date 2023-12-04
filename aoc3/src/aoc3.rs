use aoc3::{part_1_sum_parts, part_2_sum_gear_ratios, alternate_part_2_sum_gear_ratios, alternate_part_1_sum_parts};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let s1 = part_1_sum_parts(include_str!("../input.txt"));
    println!("Part 1: {}", s1);

    let s2 = part_2_sum_gear_ratios(include_str!("../input.txt"));
    println!("Part 2: {}", s2);

    let s1a = alternate_part_1_sum_parts(include_str!("../input.txt"));
    println!("Part 1 (Alternate): {}", s1a);
    
    let s2a = alternate_part_2_sum_gear_ratios(include_str!("../input.txt"));
    println!("Part 2 (Alternate): {}", s2a);
}
