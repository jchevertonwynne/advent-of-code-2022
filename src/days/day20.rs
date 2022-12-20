use std::ops::Rem;
use crate::{DayResult, IntoDayResult};
use nom::bytes::complete::tag;
use nom::combinator::all_consuming;
use nom::multi::many0;
use nom::sequence::terminated;
use nom::IResult;
use nom::character::complete as num;

pub fn run(input: &'static str, _: bool) -> anyhow::Result<DayResult> {
    let (_, numbers) = parse_numbers(input)?;
    let mut indices = (0..numbers.len()).collect::<Vec<_>>();

    let len = indices.len();

    for i in 0..indices.len() {
        let index = indices[i];
        let val = numbers[index];
        let new_pos = (index as i64 + val).rem_euclid(len as i64) as usize;

        if new_pos < index {
            for v in indices.iter_mut() {
                if *v > new_pos && *v <= index {
                    *v = (*v as i64 + 1).rem_euclid(len as i64) as usize
                }
            }
        } else {
            for v in indices.iter_mut() {
                if *v >= new_pos && *v < index {
                    *v = (*v as i64 - 1).rem_euclid(len as i64) as usize
                }
            }
        }

        indices[i] = new_pos;

        // for all middle indices update in the appropriate order
    }

    let new_tape = indices.into_iter().map(|i| numbers[i]).collect::<Vec<_>>();
    println!("{new_tape:?}");
    // numbers.sw
    ().into_result()
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<i64>> {
    all_consuming(many0(terminated(num::i64, tag("\n"))))(input)
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
