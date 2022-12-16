use crate::{DayResult, IntoDayResult};

pub fn run(input: &'static str, _: bool) -> anyhow::Result<DayResult> {
    let mut part1 = 0;
    let mut part2 = 0;

    let mut nums = [0_usize; 4];
    let mut curr = 0_usize;

    for &b in input.as_bytes() {
        match b {
            b'\n' => {
                let [a, b, c, d] = nums;
                part1 += (((a >= c) & (b <= d)) | ((c >= a) & (d <= b))) as usize;

                part2 += ((a <= c) & (b >= c)
                    | (a <= d) & (b >= d)
                    | (a <= d) & (c <= a)
                    | (b >= c) & (d >= b)) as usize;
                nums = [0; 4];
                curr = 0;
            }
            b'-' | b',' => {
                curr += 1;
            }
            b'0'..=b'9' => {
                nums[curr] *= 10;
                nums[curr] += b as usize;
            }
            _ => unreachable!(),
        }
    }

    (part1, part2).into_result()
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::DayResult;

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/04.txt"), false);
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
        let result = run(include_str!("../../input/real/04.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(450.into()),
                part2: Some(837.into()),
            }
        );
    }
}
