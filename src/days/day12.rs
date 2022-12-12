use crate::{DayResult, IntoDayResult};
use anyhow::Context;
use arrayvec::ArrayVec;
use bstr::{BStr, ByteSlice};
use fxhash::FxBuildHasher;
use std::collections::{HashSet, VecDeque};

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    let world = BStr::new(input).lines().collect::<Vec<_>>();

    let p1_start = world
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &b)| (x, y, b)))
        .find(|&(_, _, b)| b == b'S')
        .map(|(x, y, _)| Point { x, y })
        .context("failed to find start")?;

    (part_1([p1_start], &world), part_2(&world)).into_result()
}

fn part_1(start: impl IntoIterator<Item = Point>, world: &[&[u8]]) -> usize {
    let mut seen = HashSet::with_hasher(FxBuildHasher::default());

    let mut queue = VecDeque::new();
    for start in start {
        queue.push_front((start, 0));
    }

    while let Some((state, turns)) = queue.pop_front() {
        if !seen.insert(state) {
            continue;
        }

        if let Some(neighbours) = get_next(state, world) {
            for neighbour in neighbours {
                queue.push_back((neighbour, turns + 1));
            }
        } else {
            return turns + 1;
        }
    }

    unreachable!();
}

fn part_2(world: &[&[u8]]) -> usize {
    let starts = world
        .iter()
        .enumerate()
        .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, &val)| (x, y, val)))
        .filter_map(|(x, y, val)| {
            if val == b'a' {
                Some(Point { x, y })
            } else {
                None
            }
        });

    part_1(starts, world)
}

fn get_next(curr: Point, world: &[&[u8]]) -> Option<ArrayVec<Point, 4>> {
    let mut result = ArrayVec::new();

    let mut val = world[curr.y][curr.x];
    if val == b'S' {
        val = b'a';
    }

    if curr.x > 0 {
        let a = world[curr.y][curr.x - 1];
        if a == b'E' && val >= b'y' {
            return None;
        }
        if a < val || a - val <= 1 {
            result.push(Point {
                x: curr.x - 1,
                y: curr.y,
            });
        }
    }

    if curr.x + 1 < world[0].len() {
        let a = world[curr.y][curr.x + 1];
        if a == b'E' && val >= b'y' {
            return None;
        }
        if a < val || a - val <= 1 {
            result.push(Point {
                x: curr.x + 1,
                y: curr.y,
            });
        }
    }

    if curr.y > 0 {
        let a = world[curr.y - 1][curr.x];
        if a == b'E' && val >= b'y' {
            return None;
        }
        if a < val || a - val <= 1 {
            result.push(Point {
                x: curr.x,
                y: curr.y - 1,
            });
        }
    }

    if curr.y + 1 < world.len() {
        let a = world[curr.y + 1][curr.x];
        if a == b'E' && val >= b'y' {
            return None;
        }
        if a < val || a - val <= 1 {
            result.push(Point {
                x: curr.x,
                y: curr.y + 1,
            });
        }
    }

    Some(result)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::DayResult;

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/12.txt"));
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
        let result = run(include_str!("../../input/real/12.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: None,
                part2: None,
            }
        );
    }
}
