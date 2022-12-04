use anyhow::Context;
use bstr::{BStr, ByteSlice};
use nom::Slice;

use crate::{DayResult, IntoDayResult};
use crate::days::byte_slice_to_int;

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    let mut part1 = 0;
    let mut part2 = 0;

    for line in BStr::new(input).lines() {
        let h1 = line.find("-").context("failed to find the first hyphen")?;
        let mid = line.find(",").context("failed to find the comma")?;

        let second_half = line.slice(mid + 1..);
        let h2 = second_half.find("-").context("failed to find the second hyphen")?;

        let a: usize = byte_slice_to_int(line.slice(0..h1));
        let b: usize =  byte_slice_to_int( line.slice(h1 + 1..mid));
        let c: usize =  byte_slice_to_int(second_half.slice(..h2));
        let d: usize = byte_slice_to_int( second_half.slice((h2 + 1)..));

        let first = a..=b;
        let second = c..=d;

        if (first.contains(&c) && first.contains(&d))
            || (second.contains(&a) && second.contains(&b))
        {
            part1 += 1;
        }

        if first.contains(&c) || first.contains(&d) || second.contains(&a) || second.contains(&b) {
            part2 += 1;
        }
    }

    (part1, part2).into_result()
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::{DayResult};

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/04.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(2.into()),
                part2: Some(4.into()),
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/04.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(450.into()),
                part2: Some(837.into()),
            }
        );
    }
}
