use crate::days::byte_slice_to_int;
use crate::{DayResult, IntoDayResult};
use bstr::{BStr, ByteSlice};
use fxhash::FxBuildHasher;
use std::collections::HashSet;

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    let mut visited_1 = HashSet::with_hasher(FxBuildHasher::default());
    let mut visited_2 = HashSet::with_hasher(FxBuildHasher::default());
    let mut ropes = [Point::default(); 10];
    for line in BStr::new(input).lines() {
        let dir = line[0];
        let dist = byte_slice_to_int::<isize>(&line[2..]);

        for _ in 0..dist {
            let (dx, dy) = match dir {
                b'L' => (-1, 0),
                b'R' => (1, 0),
                b'U' => (0, 1),
                b'D' => (0, -1),
                _ => unreachable!(),
            };
            ropes[0].x += dx;
            ropes[0].y += dy;

            for i in 0..ropes.len() - 1 {
                let first = ropes[i];
                let mut second = &mut ropes[i + 1];
                if (first.x - second.x).abs() > 1 {
                    second.x += (first.x - second.x).signum();
                    second.y += (first.y - second.y).signum();
                }

                if (first.y - second.y).abs() > 1 {
                    second.x += (first.x - second.x).signum();
                    second.y += (first.y - second.y).signum();
                }
            }

            visited_1.insert(ropes[1]);
            visited_2.insert(ropes[9]);
        }
    }

    (visited_1.len(), visited_2.len()).into_result()
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug, Default)]
struct Point {
    x: isize,
    y: isize,
}

#[cfg(test)]
mod tests {
    use std::assert_eq;
    use super::run;
    use crate::DayResult;

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/09.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(88.into()),
                part2: Some(36.into()),
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/09.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(6044.into()),
                part2: Some(2384.into()),
            }
        );
    }
}
