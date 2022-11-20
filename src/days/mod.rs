use num::Num;
use std::ops::{AddAssign, SubAssign};

pub mod day01;
pub mod day02;

#[inline(always)]
pub fn byte_slice_to_int<T: Num + From<u8> + AddAssign + SubAssign>(slice: &[u8]) -> T {
    let mut res = T::zero();

    for &b in slice {
        let mut i: T = b.into();
        i -= b'0'.into();
        res += i;
    }

    res
}
