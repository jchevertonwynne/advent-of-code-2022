use crate::{DayResult, IntoDayResult};
use bstr::{BStr, ByteSlice};
use nom::bytes::complete::tag;
use nom::sequence::{pair, preceded};
use nom::IResult;
use num::Signed;
use std::cmp::max;
use std::ops::{Add, AddAssign, Sub};

pub fn run(input: &'static str, _: bool) -> anyhow::Result<DayResult> {
    let mut max_y = i32::MIN;

    for line in BStr::new(input).lines() {
        let line = unsafe { std::str::from_utf8_unchecked(line) };
        let (mut rem, (_, y)) = parse_coord_pair(line)?;
        max_y = max(max_y, y);

        while let Ok((_rem, (_, y))) = parse_subsequent_pair(rem) {
            max_y = max(max_y, y);
            rem = _rem;
        }
    }

    let lowest = max_y;

    max_y += 5;
    let min_x = 500 - (max_y + 5);
    let max_x = 500 + (max_y + 5);

    let mut world = World::new(min_x as usize, max_x as usize, 0, max_y as usize);

    for i in min_x..=max_x {
        world.mark(i as usize, lowest as usize + 2);
    }

    for line in BStr::new(input).lines() {
        let line = unsafe { std::str::from_utf8_unchecked(line) };
        let (mut rem, start) = parse_coord_pair(line)?;
        let mut curr: Point<_> = start.into();
        world.mark(curr.x as usize, curr.y as usize);

        while let Ok((_rem, pair)) = parse_subsequent_pair(rem) {
            let next_point: Point<_> = pair.into();
            let unit = next_point.signum(&curr);

            while curr != next_point {
                curr += unit;
                world.mark(curr.x as usize, curr.y as usize);
            }

            rem = _rem;
        }
    }

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
        if !world.is_marked(pot_down.x as usize, pot_down.y as usize) {
            sand_history.push(sand);
            sand = pot_down;
        } else if !world.is_marked(pot_left.x as usize, pot_left.y as usize) {
            sand_history.push(sand);
            sand = pot_left;
        } else if !world.is_marked(pot_right.x as usize, pot_right.y as usize) {
            sand_history.push(sand);
            sand = pot_right;
        } else {
            world.mark(sand.x as usize, sand.y as usize);
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
        if !world.is_marked(pot_down.x as usize, pot_down.y as usize) {
            sand_history.push(sand);
            sand = pot_down;
        } else if !world.is_marked(pot_left.x as usize, pot_left.y as usize) {
            sand_history.push(sand);
            sand = pot_left;
        } else if !world.is_marked(pot_right.x as usize, pot_right.y as usize) {
            sand_history.push(sand);
            sand = pot_right;
        } else if let Some(prev) = sand_history.pop() {
            world.mark(sand.x as usize, sand.y as usize);
            part2 += 1;
            sand = prev;
        } else {
            break;
        }
    }

    (part1, part2).into_result()
}

struct World {
    contents: Vec<bool>,
    x_min: usize,
    x_range: usize,
}

impl World {
    fn new(x_min: usize, x_max: usize, y_min: usize, y_max: usize) -> World {
        let x_range = x_max - x_min + 1;
        let y_range = y_max - y_min + 1;
        let contents = vec![false; x_range * y_range];

        World {
            contents,
            x_min,
            x_range,
        }
    }

    #[inline(always)]
    fn is_marked(&self, x: usize, y: usize) -> bool {
        self.contents[(x - self.x_min) + y * self.x_range]
    }

    #[inline(always)]
    fn mark(&mut self, x: usize, y: usize) {
        self.contents[(x - self.x_min) + y * self.x_range] = true;
    }
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
    #[inline(always)]
    fn signum(&self, other: &Self) -> Point<T> {
        Point {
            x: (self.x - other.x).signum(),
            y: (self.y - other.y).signum(),
        }
    }
}

impl<T: Add<Output = T> + Copy> AddAssign for Point<T> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn parse_coord_pair(input: &str) -> IResult<&str, (i32, i32)> {
    pair(
        nom::character::complete::i32,
        preceded(tag(","), nom::character::complete::i32),
    )(input)
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
        let result = run(include_str!("../../input/test/14.txt"), false);
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
        let result = run(include_str!("../../input/real/14.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(1_072.into()),
                part2: Some(24_659.into()),
            }
        );
    }
}
