use aoc1::first_and_last;

fn main() {
    println!("Testing");

    let total = include_str!("../input.txt")
        .split('\n')
        .map(aoc1::DigitIterator::new)
        .map(|d| first_and_last(d.iterate_digits()))
        .filter_map(|digits| digits.map(|(first, last)| first * 10 + last))
        .sum::<i32>();

    println!("SUM is: {}", total);
}
