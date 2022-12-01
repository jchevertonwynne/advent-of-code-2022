use num::Num;
use std::ops::{Add, Mul, Sub};

pub mod day01;

#[inline(always)]
pub fn byte_slice_to_int<T: Num + From<u8> + Add + Sub + Mul>(slice: &[u8]) -> T {
    let mut res = T::zero();

    for &b in slice {
        res = res * 10.into();
        res = res + b.into() - b'0'.into();
    }

    res
}
