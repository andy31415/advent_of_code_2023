use aoc4::{part_1_add_points, part_2_sum_cards};
use criterion::{criterion_group, criterion_main, Criterion};

fn part1(c: &mut Criterion) {
    c.bench_function("part1", |b| {
        b.iter(|| part_1_add_points(criterion::black_box(include_str!("../input.txt"))))
    });
}

fn part2(c: &mut Criterion) {
    c.bench_function("part2", |b| {
        b.iter(|| part_2_sum_cards(criterion::black_box(include_str!("../input.txt"))))
    });
}

criterion_group!(benches, part1, part2);
criterion_main!(benches);
