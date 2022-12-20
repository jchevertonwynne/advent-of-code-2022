use num::Num;
use std::ops::{Add, Mul, Sub};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;

#[inline(always)]
pub fn byte_slice_to_int<T: Num + From<u8> + Add + Sub + Mul + Copy>(slice: &[u8]) -> T {
    let mut res = T::zero();
    let ten: T = 10.into();

    for &b in slice {
        res = res * ten + b.into() - b'0'.into();
    }

    res
}
