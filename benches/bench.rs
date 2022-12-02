use advent_of_code_2022::days::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const DAY_01: &str = include_str!("../input/real/01.txt");
const DAY_02: &str = include_str!("../input/real/02.txt");

fn bench_days(c: &mut Criterion) {
    c.bench_function("day 01", |b| b.iter(|| day01::run(black_box(DAY_01))));
    c.bench_function("day 02", |b| b.iter(|| day02::run(black_box(DAY_02))));
}

criterion_group!(benches, bench_days);
criterion_main!(benches);
