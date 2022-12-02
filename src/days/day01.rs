use crate::{DayResult, IntoDayResult};

macro_rules! sort_arr {
    ( $val:expr, $arr:expr ) => {
        if $val > $arr[0] {
            $arr[2] = $arr[1];
            $arr[1] = $arr[0];
            $arr[0] = $val;
        } else if $val > $arr[1] {
            $arr[2] = $arr[1];
            $arr[1] = $val;
        } else if $val > $arr[2] {
            $arr[2] = $val;
        }
    };
}

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    let mut calorie_sums = [0; 3];

    let input = input.as_bytes();
    let mut curr_int: u32 = 0;
    let mut sum = 0;
    let mut last_newline = 0;

    for (i, &byte) in input.iter().enumerate() {
        match byte {
            b'\n' => {
                if last_newline + 1 == i {
                    sort_arr!(sum, calorie_sums);
                    sum = 0;
                } else {
                    sum += curr_int;
                    curr_int = 0;
                    last_newline = i;
                }
            }
            b'0'..=b'9' => {
                curr_int = curr_int * 10 + (byte - b'0') as u32;
            }
            _ => unreachable!(),
        }
    }

    sort_arr!(sum, calorie_sums);

    let part1 = calorie_sums[0];
    let part2 = calorie_sums.iter().sum::<u32>();

    (part1, part2).into_result()
}

pub fn run2(input: &'static str) -> anyhow::Result<DayResult> {
    let (p1, p2) = input
        .split("\n\n")
        .try_fold((0, [0u32; 4]), |(p1, mut part2_arr), group| {
            group
                .lines()
                .map(|line| line.parse::<u32>())
                .sum::<Result<u32, _>>()
                .map(|calories| {
                    part2_arr[0] = calories;
                    part2_arr.sort();
                    (std::cmp::max(p1, calories), part2_arr)
                })
        })?;

    (p1, p2.into_iter().skip(1).sum::<u32>()).into_result()
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::{Answers, DayResult};

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/01.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(Answers::U32(24000)),
                part2: Some(Answers::U32(45000)),
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/01.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(Answers::U32(69836)),
                part2: Some(Answers::U32(207968)),
            }
        );
    }
}
