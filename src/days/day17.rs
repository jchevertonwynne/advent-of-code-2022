use crate::{DayResult, IntoDayResult};
use anyhow::Context;
use std::cmp::max;
use std::collections::HashSet;
use std::ops::Add;

const SHAPES: &[&[Point]] = &[
    &[
        Point { x: 0, y: 0 },
        Point { x: 1, y: 0 },
        Point { x: 2, y: 0 },
        Point { x: 3, y: 0 },
    ],
    &[
        Point { x: 1, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: 1, y: 1 },
        Point { x: 2, y: 1 },
        Point { x: 1, y: 2 },
    ],
    &[
        Point { x: 0, y: 0 },
        Point { x: 1, y: 0 },
        Point { x: 2, y: 0 },
        Point { x: 2, y: 1 },
        Point { x: 2, y: 2 },
    ],
    &[
        Point { x: 0, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: 0, y: 2 },
        Point { x: 0, y: 3 },
    ],
    &[
        Point { x: 0, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: 1, y: 0 },
        Point { x: 1, y: 1 },
    ],
];

pub fn run(input: &'static str, _: bool) -> anyhow::Result<DayResult> {
    let mut dir = input.trim().as_bytes().iter().copied().cycle();

    let mut world = HashSet::new();

    let mut highest_settled = 0;

    for &shape in SHAPES.iter().cycle().take(2_022) {
        let offset = Point {
            x: 2,
            y: highest_settled + 3,
        };
        let mut shape = Shape { offset, shape };

        loop {
            // try and push sideways
            let dir = dir.next().context("expected a next direction")?;
            match dir {
                b'<' => {
                    if let Some(new_shape) = shape.left(&world) {
                        shape = new_shape;
                    }
                }
                b'>' => {
                    if let Some(new_shape) = shape.right(&world) {
                        shape = new_shape;
                    }
                }
                _ => unreachable!(),
            }

            // then try and move down. if fail, make solid and break
            if let Some(new_shape) = shape.down(&world) {
                shape = new_shape;
            } else {
                world.extend(shape.coords());
                highest_settled = max(
                    highest_settled,
                    shape
                        .coords()
                        .map(|c| c.y + 1)
                        .max()
                        .context("expected a non zero size")?,
                );
                break;
            }
        }
    }

    highest_settled.into_result()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Shape {
    offset: Point,
    shape: &'static [Point],
}

impl Shape {
    fn coords(&self) -> impl Iterator<Item = Point> + '_ {
        self.shape.iter().map(|&p| p + self.offset)
    }

    fn down(&self, world: &HashSet<Point>) -> Option<Shape> {
        self.shift(Point { x: 0, y: -1 }, world)
    }

    fn left(&self, world: &HashSet<Point>) -> Option<Shape> {
        self.shift(Point { x: -1, y: 0 }, world)
    }

    fn right(&self, world: &HashSet<Point>) -> Option<Shape> {
        self.shift(Point { x: 1, y: 0 }, world)
    }

    fn shift(&self, direction: Point, world: &HashSet<Point>) -> Option<Shape> {
        let shifted = *self + direction;
        let legal = shifted
            .shape
            .iter()
            .map(|&p| p + shifted.offset)
            .all(|p| p.x >= 0 && p.x < 7 && p.y >= 0 && !world.contains(&p));
        if legal {
            Some(shifted)
        } else {
            None
        }
    }
}

impl Add<Point> for Shape {
    type Output = Shape;

    fn add(self, rhs: Point) -> Self::Output {
        let Shape { offset, shape } = self;
        let offset = offset + rhs;
        Shape { offset, shape }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64,
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
        let result = run(include_str!("../../input/test/17.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(3_068.into()),
                part2: None,
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/17.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(3_127.into()),
                part2: None,
            }
        );
    }
}
