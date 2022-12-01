use num::Num;
use std::ops::{Add, Mul, Sub};

pub mod day01;

#[inline(always)]
pub fn byte_slice_to_int<T: Num + From<u8> + Add + Sub + Mul + Clone>(slice: &[u8]) -> T {
    let mut res = T::zero();

    let ten = {
        T::one()
            + T::one()
            + T::one()
            + T::one()
            + T::one()
            + T::one()
            + T::one()
            + T::one()
            + T::one()
            + T::one()
    };

    for &b in slice {
        res = res * ten.clone();
        res = res + b.into() - b'0'.into();
    }

    res
}
