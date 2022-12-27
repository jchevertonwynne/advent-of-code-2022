use crate::{DayResult, IntoDayResult};
use anyhow::Context;
use bstr::{BStr, ByteSlice};
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::fmt::{Debug, Formatter};
use std::ops::Add;

pub fn run(input: &'static str, _: bool) -> anyhow::Result<DayResult> {
    let world_width = input
        .as_bytes()
        .iter()
        .position(|&b| b == b'\n')
        .context("failed to find newline")?;
    let world_height = input.as_bytes().len() / (world_width + 1);

    let height = world_height as i64;
    let width = world_width as i64;

    let (mut walls, blizzards) = load_world(input);
    walls.insert(Point { x: 1, y: -1 });
    walls.insert(Point {
        x: width - 2,
        y: height,
    });

    let start = Point { x: 1, y: 0 };

    let goal = Point {
        x: width - 2,
        y: height - 1,
    };
    let part1 = traverse(&walls, &blizzards, start, goal, width, height, 0);
    let back = traverse(&walls, &blizzards, goal, start, width, height, part1);
    let part2 = traverse(&walls, &blizzards, start, goal, width, height, part1 + back);

    (part1, part1 + back + part2).into_result()
}

fn traverse(
    walls: &HashSet<Point>,
    blizzards: &[Blizzard],
    start: Point,
    goal: Point,
    width: i64,
    height: i64,
    offset: i64,
) -> i64 {
    let mut seen = HashSet::new();

    let cycle = (width - 2) * (height - 2);

    let mut states: BinaryHeap<Reverse<AStarEntry>> = BinaryHeap::new();

    states.push(Reverse(AStarEntry {
        point: start,
        turns: 0,
        est: start.manhattan(&goal),
    }));

    while let Some(Reverse(AStarEntry {
        point: position,
        turns: turn,
        est,
    })) = states.pop()
    {
        // wait in place if possible
        if !blizzards
            .iter()
            .any(|b| b.offset_by(turn + 1 + offset, width, height) == position)
        {
            states.push(Reverse(AStarEntry {
                point: position,
                turns: turn + 1,
                est,
            }));
        }

        // 4 dirs
        let dirs = [
            Point { x: -1, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: -1 },
            Point { x: 0, y: 1 },
        ];
        for dir in dirs {
            let new_position = position + dir;

            if walls.contains(&new_position) {
                continue;
            }

            if blizzards
                .iter()
                .any(|b| b.offset_by(turn + 1 + offset, width, height) == new_position)
            {
                continue;
            }

            if !seen.insert((new_position, ((turn + 1) % cycle))) {
                continue;
            }

            if new_position == goal {
                return turn + 1;
            }

            states.push(Reverse(AStarEntry {
                point: new_position,
                turns: turn + 1,
                est: new_position.manhattan(&goal),
            }));
        }
    }

    unreachable!()
}

fn load_world(input: &str) -> (HashSet<Point>, Vec<Blizzard>) {
    let mut walls = HashSet::new();
    let mut blizzards = Vec::new();

    for (y, line) in BStr::new(input).lines().enumerate() {
        for (x, b) in line.iter().copied().enumerate() {
            let x = x as i64;
            let y = y as i64;
            let point = Point { x, y };
            match b {
                b'#' => {
                    walls.insert(point);
                }
                b'>' => {
                    let dir = Point { x: 1, y: 0 };
                    blizzards.push(Blizzard {
                        position: point,
                        r#move: dir,
                    });
                }
                b'<' => {
                    let dir = Point { x: -1, y: 0 };
                    blizzards.push(Blizzard {
                        position: point,
                        r#move: dir,
                    });
                }
                b'^' => {
                    let dir = Point { x: 0, y: -1 };
                    blizzards.push(Blizzard {
                        position: point,
                        r#move: dir,
                    });
                }
                b'v' => {
                    let dir = Point { x: 0, y: 1 };
                    blizzards.push(Blizzard {
                        position: point,
                        r#move: dir,
                    });
                }
                b'.' => {}
                _ => unreachable!(),
            }
        }
    }

    (walls, blizzards)
}

#[derive(Eq, PartialEq)]
struct AStarEntry {
    point: Point,
    turns: i64,
    est: i64,
}

impl PartialOrd<Self> for AStarEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AStarEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.turns + self.est).cmp(&(other.turns + other.est))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Blizzard {
    position: Point,
    r#move: Point,
}

impl Blizzard {
    fn offset_by(&self, dist: i64, width: i64, height: i64) -> Point {
        let x = ((self.position.x - 1) + dist * self.r#move.x).rem_euclid(width - 2) + 1;
        let y = ((self.position.y - 1) + dist * self.r#move.y).rem_euclid(height - 2) + 1;
        Point { x, y }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Point {
    x: i64,
    y: i64,
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}, {}}}", self.x, self.y)
    }
}

impl Point {
    fn manhattan(&self, other: &Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::DayResult;

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/24.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(18.into()),
                part2: Some(54.into()),
            }
        );
    }

    // #[test]
    // fn test_answers() {
    //     let result = run(include_str!("../../input/real/24.txt"), false);
    //     assert_eq!(
    //         result.unwrap(),
    //         DayResult {
    //             part1: Some(311.into()),
    //             part2: Some(869.into()),
    //         }
    //     );
    // }
}
