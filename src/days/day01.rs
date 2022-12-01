use crate::days::byte_slice_to_int;
use crate::{DayResult, IntoDayResult};
use bstr::ByteSlice;

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    let mut cal_sums = [0; 4];

    let input = bstr::BStr::new(input);

    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            cal_sums[0] = sum;
            cal_sums.sort();
            sum = 0;
        } else {
            sum += byte_slice_to_int::<u64>(line);
        }
    }

    cal_sums[0] = sum;
    cal_sums.sort();

    let part1 = cal_sums[3];
    let part2 = cal_sums[1] + cal_sums[2] + cal_sums[3];

    (part1, part2).into_result()
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::{Answers, DayResult};

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/01.txt"));
        assert!(matches!(
            result,
            Ok(DayResult {
                part1: Some(Answers::U64(69836)),
                part2: Some(Answers::U64(207968)),
            })
        ));
    }
}
