use crate::{DayResult, IntoDayResult};
use nom::bytes::complete::tag;
use nom::character::complete as num;
use nom::combinator::all_consuming;
use nom::multi::many0;
use nom::sequence::terminated;
use nom::IResult;
use std::ops::Rem;

pub fn run(input: &'static str, _: bool) -> anyhow::Result<DayResult> {
    let (_, numbers) = parse_numbers(input)?;

    let part1 = solve(numbers.iter().copied(), 1);
    let part2 = solve(numbers.iter().map(|n| n * 811589153), 10);

    (part1, part2).into_result()
}

fn solve(nums: impl IntoIterator<Item = i64>, mixes: usize) -> i64 {
    let numbers = nums.into_iter().collect::<Vec<_>>();
    // vec of original index to new index
    let mut indices = (0..numbers.len()).collect::<Vec<_>>();
    // vec of (val, orig_index)
    let mut numbers = numbers
        .into_iter()
        .enumerate()
        .map(|(orig_index, val)| Entry { val, orig_index })
        .collect::<Vec<_>>();
    // when vec 2 is updated, also update the other vec's index accordingly

    let zero_ind = numbers.iter().position(|e| e.val == 0).unwrap();

    let len = indices.len() as i64;

    for _ in 0..mixes {
        for i in 0..indices.len() {
            let index = indices[i];
            let entry = numbers[index];

            let sig = entry.val.signum();
            let mut index = index as i64;

            for _ in 0..entry.val.abs().rem(len - 1) {
                let a = index as usize;
                let curr_ind_a = indices[numbers[a].orig_index];
                let mut new_curr_ind_a = curr_ind_a as i64 + sig;
                if new_curr_ind_a == len {
                    new_curr_ind_a = 0;
                } else if new_curr_ind_a == -1 {
                    new_curr_ind_a = len - 1;
                };
                indices[numbers[a].orig_index] = new_curr_ind_a as usize;

                let mut b = index + sig;
                if b < 0 {
                    b = len - 1;
                } else if b >= len {
                    b = 0;
                }
                let b = b as usize;
                let curr_ind_b = indices[numbers[b].orig_index];
                let mut new_curr_ind_b = curr_ind_b as i64 - sig;
                if new_curr_ind_b == len {
                    new_curr_ind_b = 0;
                } else if new_curr_ind_b == -1 {
                    new_curr_ind_b = len - 1;
                };
                indices[numbers[b].orig_index] = new_curr_ind_b as usize;

                numbers.swap(curr_ind_a, curr_ind_b);
                index = (index + sig).rem_euclid(len);
            }
        }
    }

    let zero_ind = indices[zero_ind];

    numbers[(zero_ind + 1000) % numbers.len()].val
        + numbers[(zero_ind + 2000) % numbers.len()].val
        + numbers[(zero_ind + 3000) % numbers.len()].val
}

/*
Initial arrangement:
1, 2, -3, 3, -2, 0, 4

1 moves between 2 and -3:
2, 1, -3, 3, -2, 0, 4

2 moves between -3 and 3:
1, -3, 2, 3, -2, 0, 4

-3 moves between -2 and 0:
1, 2, 3, -2, -3, 0, 4

3 moves between 0 and 4:
1, 2, -2, -3, 0, 3, 4

-2 moves between 4 and 1:
1, 2, -3, 0, 3, 4, -2

0 does not move:
1, 2, -3, 0, 3, 4, -2

4 moves between -3 and 0:
1, 2, -3, 4, 0, 3, -2
*/

fn parse_numbers(input: &str) -> IResult<&str, Vec<i64>> {
    all_consuming(many0(terminated(num::i64, tag("\n"))))(input)
}

#[derive(Debug, Clone, Copy)]
struct Entry {
    val: i64,
    orig_index: usize,
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::DayResult;

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/20.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: None,
                part2: None,
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/20.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: None,
                part2: None,
            }
        );
    }
}
