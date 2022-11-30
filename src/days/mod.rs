use num::Num;
use std::ops::{Add, Sub};

pub mod day01;
pub mod day02;
pub mod day03;

#[inline(always)]
pub fn byte_slice_to_int<T: Num + From<u8> + Add + Sub>(slice: &[u8]) -> T {
    let mut res = T::zero();

    for &b in slice {
        res = res + b.into() - b'0'.into();
    }

    res
}
