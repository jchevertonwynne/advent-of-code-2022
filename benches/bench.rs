use advent_of_code_2022::days::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_days(c: &mut Criterion) {
    // c.bench_function("day 01", |b| {
    //     b.iter(|| day01::run(black_box(include_str!("../input/real/01.txt")), true))
    // });
    // c.bench_function("day 02", |b| {
    //     b.iter(|| day02::run(black_box(include_str!("../input/real/02.txt")), true))
    // });
    // c.bench_function("day 03", |b| {
    //     b.iter(|| day03::run(black_box(include_str!("../input/real/03.txt")), true))
    // });
    // c.bench_function("day 04", |b| {
    //     b.iter(|| day04::run(black_box(include_str!("../input/real/04.txt")), true))
    // });
    // c.bench_function("day 05", |b| {
    //     b.iter(|| day05::run(black_box(include_str!("../input/real/05.txt")), true))
    // });
    // c.bench_function("day 06", |b| {
    //     b.iter(|| day06::run(black_box(include_str!("../input/real/06.txt")), true))
    // });
    // c.bench_function("day 07", |b| {
    //     b.iter(|| day07::run(black_box(include_str!("../input/real/07.txt")), true))
    // });
    // c.bench_function("day 08", |b| {
    //     b.iter(|| day08::run(black_box(include_str!("../input/real/08.txt")), true))
    // });
    // c.bench_function("day 09", |b| {
    //     b.iter(|| day09::run(black_box(include_str!("../input/real/09.txt")), true))
    // });
    // c.bench_function("day 10", |b| {
    //     b.iter(|| day10::run(black_box(include_str!("../input/real/10.txt")), true))
    // });
    // c.bench_function("day 11", |b| {
    //     b.iter(|| day11::run(black_box(include_str!("../input/real/11.txt")), true))
    // });
    // c.bench_function("day 12", |b| {
    //     b.iter(|| day12::run(black_box(include_str!("../input/real/12.txt")), true))
    // });
    // c.bench_function("day 13", |b| {
    //     b.iter(|| day13::run(black_box(include_str!("../input/real/13.txt")), true))
    // });
    // c.bench_function("day 14", |b| {
    //     b.iter(|| day14::run(black_box(include_str!("../input/real/14.txt")), true))
    // });
    c.bench_function("day 15", |b| {
        b.iter(|| day15::run(black_box(include_str!("../input/real/14.txt")), true))
    });
}

criterion_group!(benches, bench_days);
criterion_main!(benches);
