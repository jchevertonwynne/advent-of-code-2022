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

    let mut part2: usize = 0;

    for (col, line) in trees.iter().enumerate() {
        for (row, &home) in line.iter().enumerate() {
            let mut a = 0;
            for i in (0..(col)).rev() {
                let t = trees[i][row];

                a += 1;

                if t >= home {
                    break;
                }
            }

            let mut b = 0;
            for i in col + 1..trees.len() {
                let t = trees[i][row];

                b += 1;

                if t >= home {
                    break;
                }
            }

            let mut c = 0;
            for i in (0..(row)).rev() {
                let t = trees[col][i];

                c += 1;

                if t >= home {
                    break;
                }
            }

            let mut d = 0;
            for i in row + 1..trees[0].len() {
                let t = trees[col][i];

                d += 1;

                if t >= home {
                    break;
                }
            }

            part2 = std::cmp::max(part2, a * b * c * d);
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
        let result = run(include_str!("../../input/test/08.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(21.into()),
                part2: Some(8.into()),
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/08.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(1669.into()),
                part2: Some(331344.into()),
            }
        );
    }
}
