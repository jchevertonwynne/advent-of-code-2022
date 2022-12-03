use crate::{DayResult, IntoDayResult};
use bstr::{BStr, ByteSlice};

const LOOKUP: [u64; 58] = [
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    20, 21, 22, 23, 24, 25, 26,
];

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    let mut part1 = 0;
    let mut part2 = 0;

    let mut grouped = u64::MAX;

    let input = BStr::new(input);

    for (i, line) in input.lines().enumerate() {
        let (a, b) = line.split_at(line.len() / 2);
        let (a, b) = a
            .iter()
            .zip(b.iter())
            .fold((0_u64, 0_u64), |(acc_a, acc_b), (&a, &b)| {
                (acc_a | (1 << (a - b'A')), acc_b | 1 << (b - b'A'))
            });

        part1 += LOOKUP[(a & b).trailing_zeros() as usize];

        grouped &= a | b;

        if i % 3 == 2 {
            part2 += LOOKUP[grouped.trailing_zeros() as usize];
            grouped = u64::MAX;
        }
    }

    (part1, part2).into_result()
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
                part1: Some(Answers::U64(157)),
                part2: Some(Answers::U64(70)),
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/03.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(Answers::U64(8085)),
                part2: Some(Answers::U64(2515)),
            }
        );
    }
}
