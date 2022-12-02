use crate::{DayResult, IntoDayResult};

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    let mut part1: u32 = 0;
    let mut part2: u32 = 0;

    let input = input.as_bytes();

    for (first, second) in (0..input.len())
        .step_by(4)
        .map(|i| (input[i], input[i + 2]))
    {
        let opp = (first - b'A') as i8;
        let me_or_goal = (second - b'X') as i8;
        let res = (me_or_goal - opp + 1).rem_euclid(3) * 3;
        let should_achieve = (me_or_goal - 1 + opp).rem_euclid(3) + 1;
        part1 += (me_or_goal + 1) as u32 + res as u32;
        part2 += should_achieve as u32 + (me_or_goal * 3) as u32;
    }

    (part1, part2).into_result()
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::{Answers, DayResult};

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/02.txt"));
        println!("{:?}", result);
        assert!(matches!(
            result,
            Ok(DayResult {
                part1: Some(Answers::U32(15)),
                part2: Some(Answers::U32(12)),
            })
        ));
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/02.txt"));
        println!("{:?}", result);
        assert!(matches!(
            result,
            Ok(DayResult {
                part1: Some(Answers::U32(9241)),
                part2: Some(Answers::U32(14610)),
            })
        ));
    }
}
