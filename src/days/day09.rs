use crate::days::byte_slice_to_int;
use crate::{DayResult, IntoDayResult};
use bstr::{BStr, ByteSlice};

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    const X: usize = 332;
    const X_OFF: isize = 50;
    const Y: usize = 450;
    const Y_OFF: isize = 50;
    let mut part1 = 0;
    let mut part2 = 0;
    let mut seen_1 = [0; X * Y];
    let mut seen_2 = [0; X * Y];
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

            if seen_1[(ropes[1].x + X_OFF) as usize * X + (ropes[1].y + Y_OFF) as usize] == 0 {
                part1 += 1;
            }
            seen_1[(ropes[1].x + X_OFF) as usize * X + (ropes[1].y + Y_OFF) as usize] += 1;
            if seen_2[(ropes[9].x + X_OFF) as usize * X + (ropes[9].y + Y_OFF) as usize] == 0 {
                part2 += 1;
            }
            seen_2[(ropes[9].x + X_OFF) as usize * X + (ropes[9].y + Y_OFF) as usize] += 1;
        }
    }

    (part1, part2).into_result()
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug, Default)]
struct Point {
    x: isize,
    y: isize,
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::DayResult;
    use std::assert_eq;

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
