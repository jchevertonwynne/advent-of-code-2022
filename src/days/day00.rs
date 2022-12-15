use crate::{DayResult, IntoDayResult};

pub fn run(input: &'static str, is_test: bool) -> anyhow::Result<DayResult> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::{DayResult};

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/00.txt"), false);
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
        let result = run(include_str!("../../input/real/00.txt"), true);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: None,
                part2: None,
            }
        );
    }
}
