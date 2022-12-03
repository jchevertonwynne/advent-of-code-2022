use crate::{DayResult, IntoDayResult};
use anyhow::Context;
use bstr::{BStr, ByteSlice};

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    let mut part1 = 0;
    let mut part2 = 0;

    let mut grouped = [true; 52];

    let input = BStr::new(input);

    for (i, line) in input.lines().enumerate() {
        let (a, b) = line.split_at(line.len() / 2);
        let mut seen1 = [false; 52];
        for &item in a {
            seen1[to_position(item) - 1] = true;
        }
        let mut seen2 = [false; 52];
        for &item in b {
            seen2[to_position(item) - 1] = true;
        }

        for (i, (a, (b, c))) in grouped
            .iter_mut()
            .zip(seen1.iter().zip(seen2.iter()))
            .enumerate()
        {
            *a &= *b | *c;
            if *b && *c {
                part1 += i + 1;
            }
        }

        if i % 3 == 2 {
            part2 += grouped
                .iter()
                .position(|&b| b)
                .context("no index remained true")?
                + 1;
            grouped = [true; 52];
        }
    }

    (part1, part2).into_result()
}

fn to_position(b: u8) -> usize {
    let b = match b {
        b'a'..=b'z' => b - b'a' + 1,
        b'A'..=b'Z' => b - b'A' + 27,
        _ => unreachable!(),
    };
    b as usize
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::{Answers, DayResult};

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/03.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(Answers::Usize(157)),
                part2: Some(Answers::Usize(70)),
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/03.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(Answers::Usize(8085)),
                part2: Some(Answers::Usize(2515)),
            }
        );
    }
}
