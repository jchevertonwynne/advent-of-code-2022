use crate::{DayResult, IntoDayResult};
use bstr::{BStr, ByteSlice};

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    let trees = BStr::new(input).lines().collect::<Vec<_>>();

    let mut visible = vec![false; trees.len() * trees[0].len()];

    for (col, line) in trees.iter().enumerate() {
        let mut smallest: u8 = 0;
        for (row, &char) in line.iter().enumerate() {
            if char > smallest {
                visible[col * trees.len() + row] = true;
                smallest = char;
            }
        }

        let mut smallest: u8 = 0;
        for (row, &char) in line.iter().enumerate().rev() {
            if char > smallest {
                visible[col * trees.len() + row] = true;
                smallest = char;
            }
        }
    }

    for (row, _) in trees[0].iter().enumerate() {
        let mut smallest: u8 = 0;
        for (col, _) in trees.iter().enumerate() {
            let char = trees[col][row];
            if char > smallest {
                visible[col * trees.len() + row] = true;
                smallest = char;
            }
        }

        let mut smallest: u8 = 0;
        for (col, _) in trees.iter().enumerate().rev() {
            let char = trees[col][row];
            if char > smallest {
                visible[col * trees.len() + row] = true;
                smallest = char;
            }
        }
    }

    let part1 = visible.iter().filter(|b| **b).count();

    for (col, line) in trees.iter().enumerate().skip(1) {
        for (row, &char) in line.iter().enumerate().skip(1) {
            let mut smallest: u8 = 0;
            let mut a = 0;
            for i in (0..(col)).rev() {
                if
            }
        }
    }

    part1.into_result()
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::DayResult;

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/08.txt"));
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
        let result = run(include_str!("../../input/real/08.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: None,
                part2: None,
            }
        );
    }
}
