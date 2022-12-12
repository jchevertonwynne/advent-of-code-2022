use crate::{DayResult, IntoDayResult};
use anyhow::Context;
use arrayvec::ArrayVec;
use fxhash::FxBuildHasher;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::hash::{BuildHasher, Hash};

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    let mut seen = HashSet::with_hasher(FxBuildHasher::default());
    let mut queue = BinaryHeap::new();

    let input = input.as_bytes();
    let line_length = input
        .iter()
        .position(|&b| b == b'\n')
        .context("expected a newline")?
        + 1;

    let p1_start = input
        .iter()
        .position(|&b| b == b'S')
        .map(|ind| Point {
            x: ind % line_length,
            y: ind / line_length,
        })
        .context("failed to find the start")?;

    let end = input
        .iter()
        .position(|&b| b == b'E')
        .map(|ind| Point {
            x: ind % line_length,
            y: ind / line_length,
        })
        .context("failed to find the start")?;

    (
        part_1([p1_start], end, input, line_length, &mut seen, &mut queue),
        part_2(input, end, line_length, &mut seen, &mut queue),
    )
        .into_result()
}

#[derive(Eq, PartialEq)]
struct Entry {
    state: Point,
    dist: usize,
    est: usize,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.dist + self.est).cmp(&(other.dist + other.est))
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part_1(
    start: impl IntoIterator<Item = Point>,
    end: Point,
    input: &[u8],
    line_length: usize,
    seen: &mut HashSet<Point, impl BuildHasher>,
    queue: &mut BinaryHeap<Reverse<Entry>>,
) -> usize {
    seen.clear();
    queue.clear();
    for item in start {
        queue.push(Reverse(Entry {
            state: item,
            dist: 0,
            est: item.dist(end),
        }));
    }

    while let Some(Reverse(Entry { state, dist, .. })) = queue.pop() {
        if !seen.insert(state) {
            continue;
        }

        if let Some(neighbours) = get_next(state, input, line_length) {
            for neighbour in neighbours {
                queue.push(Reverse(Entry {
                    state: neighbour,
                    dist: dist + 1,
                    est: neighbour.dist(end),
                }));
            }
        } else {
            return dist + 1;
        }
    }

    unreachable!();
}

fn part_2(
    input: &[u8],
    end: Point,
    line_length: usize,
    seen: &mut HashSet<Point, impl BuildHasher>,
    queue: &mut BinaryHeap<Reverse<Entry>>,
) -> usize {
    let starts = input.iter().enumerate().filter_map(|(i, &b)| {
        if b == b'a' {
            let x = i % line_length;
            let y = i / line_length;
            Some(Point { x, y })
        } else {
            None
        }
    });

    part_1(starts, end, input, line_length, seen, queue)
}

fn get_next(curr: Point, input: &[u8], line_length: usize) -> Option<ArrayVec<Point, 4>> {
    let mut result = ArrayVec::new();

    let mut val = input[curr.y * line_length + curr.x];
    if val == b'S' {
        val = b'a';
    }

    if curr.x > 0 {
        let a = input[curr.y * line_length + curr.x - 1];
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

    if curr.x + 1 < line_length {
        let a = input[curr.y * line_length + curr.x + 1];
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
        let a = input[(curr.y - 1) * line_length + curr.x];
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

    if curr.y + 1 < input.len() / line_length {
        let a = input[(curr.y + 1) * line_length + curr.x];
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

impl Point {
    fn dist(self, other: Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
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
                part1: Some(31.into()),
                part2: Some(29.into()),
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/12.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(408.into()),
                part2: Some(399.into()),
            }
        );
    }
}
