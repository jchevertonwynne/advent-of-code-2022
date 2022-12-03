use crate::{DayResult, IntoDayResult};

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::{Answers, DayResult};

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/01.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: None,
                part2: None,
            }
        );
    }
}
