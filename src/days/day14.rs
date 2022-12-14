use crate::{DayResult, IntoDayResult};
use anyhow::Context;
use fxhash::FxBuildHasher;
use nom::bytes::complete::tag;
use nom::sequence::{preceded, tuple};
use nom::IResult;
use num::Signed;
use std::collections::HashSet;
use std::ops::{Add, AddAssign, Sub};

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    let mut world: HashSet<Point<i32>, _> = HashSet::with_hasher(FxBuildHasher::default());

    for line in input.lines() {
        let (mut rem, start) = parse_coord_pair(line)?;
        let mut curr: Point<_> = start.into();
        world.insert(curr);

        while let Ok((_rem, pair)) = parse_subsequent_pair(rem) {
            let next_point: Point<_> = pair.into();

            while curr != next_point {
                curr += next_point.signum(&curr);
                world.insert(curr);
            }

            rem = _rem;
        }
    }

    let lowest = world
        .iter()
        .map(|p| p.y)
        .max()
        .context("set should be non empty")?;

    let mut sand = Point::<i32> { x: 500, y: 0 };
    let mut sand_history = vec![sand];

    let mut part1 = 0;
    while sand.y < lowest {
        let pot_down = Point {
            x: sand.x,
            y: sand.y + 1,
        };
        let pot_left = Point {
            x: sand.x - 1,
            y: sand.y + 1,
        };
        let pot_right = Point {
            x: sand.x + 1,
            y: sand.y + 1,
        };
        if !world.contains(&pot_down) {
            sand_history.push(sand);
            sand = pot_down;
        } else if !world.contains(&pot_left) {
            sand_history.push(sand);
            sand = pot_left;
        } else if !world.contains(&pot_right) {
            sand_history.push(sand);
            sand = pot_right;
        } else {
            world.insert(sand);
            part1 += 1;
            if let Some(prev) = sand_history.pop() {
                sand = prev;
            } else {
                sand = Point { x: 500, y: 0 };
            }
        }
    }

    let mut part2 = part1;
    loop {
        if sand.y == lowest + 1 {
            if let Some(prev) = sand_history.pop() {
                world.insert(sand);
                part2 += 1;
                sand = prev;
            } else {
                break;
            }
            continue;
        }
        let pot_down = Point {
            x: sand.x,
            y: sand.y + 1,
        };
        let pot_left = Point {
            x: sand.x - 1,
            y: sand.y + 1,
        };
        let pot_right = Point {
            x: sand.x + 1,
            y: sand.y + 1,
        };
        if !world.contains(&pot_down) {
            sand_history.push(sand);
            sand = pot_down;
        } else if !world.contains(&pot_left) {
            sand_history.push(sand);
            sand = pot_left;
        } else if !world.contains(&pot_right) {
            sand_history.push(sand);
            sand = pot_right;
        } else if let Some(prev) = sand_history.pop() {
            world.insert(sand);
            part2 += 1;
            sand = prev;
        } else {
            break;
        }
    }

    (part1, part2).into_result()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> From<(T, T)> for Point<T> {
    fn from((x, y): (T, T)) -> Self {
        Point { x, y }
    }
}

impl<T: Signed + Sub<Output = T> + Copy> Point<T> {
    fn signum(&self, other: &Self) -> Point<T> {
        Point {
            x: (self.x - other.x).signum(),
            y: (self.y - other.y).signum(),
        }
    }
}

impl<T: Add<Output = T> + Copy> AddAssign for Point<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn parse_coord_pair(input: &str) -> IResult<&str, (i32, i32)> {
    tuple((
        nom::character::complete::i32,
        preceded(tag(","), nom::character::complete::i32),
    ))(input)
}

fn parse_subsequent_pair(input: &str) -> IResult<&str, (i32, i32)> {
    preceded(tag(" -> "), parse_coord_pair)(input)
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::DayResult;

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/14.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(24.into()),
                part2: Some(93.into()),
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/14.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(1072.into()),
                part2: Some(24659.into()),
            }
        );
    }
}
