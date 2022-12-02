use crate::{DayResult, IntoDayResult};

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
                    if sum > calorie_sums[0] {
                        calorie_sums[2] = calorie_sums[1];
                        calorie_sums[1] = calorie_sums[0];
                        calorie_sums[0] = sum;
                    } else if sum > calorie_sums[1] {
                        calorie_sums[2] = calorie_sums[1];
                        calorie_sums[1] = sum;
                    } else if sum > calorie_sums[2] {
                        calorie_sums[2] = sum;
                    }
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

    let part1 = calorie_sums[0];
    let part2 = calorie_sums.iter().sum::<u32>();

    (part1, part2).into_result()
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::{Answers, DayResult};

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/01.txt"));
        println!("{:?}", result);
        assert!(matches!(
            result,
            Ok(DayResult {
                part1: Some(Answers::U32(69836)),
                part2: Some(Answers::U32(207968)),
            })
        ));
    }
}
