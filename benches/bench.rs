use advent_of_code_2022::{
    days::{day01, day02},
    Answers, DayResult,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const DAY_01: &str = include_str!("../input/real/01.txt");
const DAY_02: &str = include_str!("../input/real/02.txt");

fn bench_days(c: &mut Criterion) {
    c.bench_function("day 01", |b| {
        b.iter(|| day01::run(black_box(DAY_01)).unwrap())
    });

    c.bench_function("day 02", |b| {
        b.iter(|| day02::run(black_box(DAY_02)).unwrap())
    });

    let lines: Vec<&str> = DAY_02.lines().collect();

    c.bench_function("day 02 lines", |b| {
        b.iter(|| run(black_box(&lines)).unwrap())
    });
}

criterion_group!(benches, bench_days);
criterion_main!(benches);

pub fn run(input: &[&str]) -> anyhow::Result<DayResult> {
    let mut horizontal: i32 = 0;
    let mut part1depth_and_part2aim: i32 = 0;
    let mut part2depth: i32 = 0;

    for line in input.iter().map(|line| line.as_bytes()) {
        let delta = Into::<i32>::into(line[line.len() - 1] - b'0');
        horizontal += delta * ((line[0] == b'f') as i32);
        part2depth += (part1depth_and_part2aim * delta) * ((line[0] == b'f') as i32);
        part1depth_and_part2aim +=
            delta * (((line[0] == b'd') as i32) - ((line[0] == b'u') as i32));
    }

    Ok(DayResult {
        part1: Some(Answers::I32(horizontal * part1depth_and_part2aim)),
        part2: Some(Answers::I32(horizontal * part2depth)),
    })
}
