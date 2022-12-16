use crate::{DayResult, IntoDayResult};
use anyhow::Context;

pub fn run(input: &'static str, _: bool) -> anyhow::Result<DayResult> {
    let input = input.as_bytes();

    let part1 = find_unique_len(4, input).context("exp an an answer")?;
    let part2 = find_unique_len(14, input).context("exp an an answer")?;

    (part1, part2).into_result()
}

fn find_unique_len(len: usize, src: &[u8]) -> Option<usize> {
    src.windows(len)
        .enumerate()
        .find(|&(_, arr)| {
            let mut bits = 0_u64;
            for &item in arr {
                let item = 1 << (item - b'a');
                if bits & item != 0 {
                    return false;
                }
                bits |= item;
            }
            true
        })
        .map(|(i, _)| i + len)
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::DayResult;

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/06.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(7.into()),
                part2: Some(19.into()),
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/06.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(1892.into()),
                part2: Some(2313.into()),
            }
        );
    }
}
