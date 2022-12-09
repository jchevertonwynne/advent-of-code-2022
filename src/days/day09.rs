use std::collections::HashSet;
use crate::{DayResult, IntoDayResult};

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    let part1 = simulate_ropes2::<2>(input)?;
    let part2 = simulate_ropes2::<10>(input)?;
    (part1, part2).into_result()
}

fn simulate_ropes2<const N: usize>(input: &str) -> anyhow::Result<usize> {
    let mut visited = HashSet::new();
    let mut ropes = [Point{x: 0, y: 0}; N];
    for line in input.lines() {
        let (dir, dist) = line.split_at(2);
        let dir = dir.as_bytes()[0];
        let dist = dist.parse::<isize>()?;

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
                if (ropes[i].x - ropes[i + 1].x).abs() > 1 {
                    ropes[i + 1].x += if ropes[i].x - ropes[i + 1].x > 0 { 1 } else { -1 };
                    if ropes[i].y != ropes[i + 1].y {
                        ropes[i + 1].y = ropes[i].y;
                    }
                }
                if (ropes[i].y - ropes[i + 1].y).abs() > 1 {
                    ropes[i + 1].y += if ropes[i].y - ropes[i + 1].y > 0 { 1 } else { -1 };
                    if ropes[i].x != ropes[i + 1].x {
                        ropes[i + 1].x = ropes[i].x;
                    }
                }
            }

            visited.insert(ropes[N - 1]);
        }
    }
    Ok(visited.len())
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::DayResult;

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/09.txt"));
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
        let result = run(include_str!("../../input/real/09.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: None,
                part2: None,
            }
        );
    }
}
