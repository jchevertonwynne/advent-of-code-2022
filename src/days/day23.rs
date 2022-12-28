use crate::{DayResult, IntoDayResult};
use bstr::{BStr, ByteSlice};
use fxhash::FxBuildHasher;
use itertools::{Itertools, MinMaxResult};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::hash::BuildHasher;

pub fn run(input: &'static str, _: bool) -> anyhow::Result<DayResult> {
    let mut elves = BStr::new(input)
        .lines()
        .enumerate()
        .flat_map(|(j, line)| {
            line.iter().enumerate().filter_map(move |(i, &b)| {
                if b == b'#' {
                    let x = i as i64;
                    let y = j as i64;
                    Some(Point { x, y })
                } else {
                    None
                }
            })
        })
        .collect::<HashSet<Point, FxBuildHasher>>();

    let mut new_elves = HashSet::with_capacity_and_hasher(elves.len(), FxBuildHasher::default());

    let mut choice_counts: HashMap<Point, usize> = HashMap::new();
    let mut choices: HashMap<Point, Point> = HashMap::new();

    for i in 0..10 {
        choice_counts.clear();
        choices.clear();
        for &elf in &elves {
            if !elf.neighbours().into_iter().any(|n| elves.contains(&n)) {
                choices.insert(elf, elf);
            } else if let Some(want) = choose_wanted_move(elf, i, &elves) {
                *choice_counts.entry(want).or_default() += 1;
                choices.insert(elf, want);
            } else {
                choices.insert(elf, elf);
            }
        }

        new_elves.clear();

        for (&elf, &choice) in &choices {
            if choice_counts.get(&choice).copied().unwrap_or(1) == 1 {
                new_elves.insert(choice);
            } else {
                new_elves.insert(elf);
            }
        }

        std::mem::swap(&mut elves, &mut new_elves);
    }

    let f = |f2: Box<dyn Fn(&Point) -> i64>| match elves.iter().map(f2).minmax() {
        MinMaxResult::NoElements => unreachable!(),
        MinMaxResult::OneElement(_) => unreachable!(),
        MinMaxResult::MinMax(a, b) => (a, b),
    };

    let (min_x, max_x) = f(Box::new(|e: &Point| e.x));
    let (min_y, max_y) = f(Box::new(|e: &Point| e.y));

    let part1 = ((max_y - min_y + 1) * (max_x - min_x + 1)) - elves.len() as i64;

    let mut part2 = 10;
    for i in 10.. {
        choice_counts.clear();
        choices.clear();
        for &elf in &elves {
            if !elf.neighbours().into_iter().any(|n| elves.contains(&n)) {
                choices.insert(elf, elf);
            } else if let Some(want) = choose_wanted_move(elf, i, &elves) {
                *choice_counts.entry(want).or_default() += 1;
                choices.insert(elf, want);
            } else {
                choices.insert(elf, elf);
            }
        }

        new_elves.clear();

        for (&elf, &choice) in &choices {
            if let Some(1) = choice_counts.get(&choice).copied() {
                new_elves.insert(choice);
            } else {
                new_elves.insert(elf);
            }
        }

        if elves == new_elves {
            break;
        }

        std::mem::swap(&mut elves, &mut new_elves);
        part2 = i + 2;
    }

    (part1, part2).into_result()
}

fn choose_wanted_move(
    point: Point,
    rot: i32,
    elves: &HashSet<Point, impl BuildHasher>,
) -> Option<Point> {
    let neighbours = point.neighbours();
    let mut has_elf = [false; 8];
    for (e, n) in has_elf.iter_mut().zip(neighbours.iter()) {
        *e = elves.contains(n);
    }
    for i in rot..rot + 4 {
        let i = i % 4;
        let (to_check, sol) = match i {
            0 => {
                // north = -y
                (
                    [0, 3, 6],
                    Point {
                        x: point.x,
                        y: point.y - 1,
                    },
                )
            }
            1 => {
                // south = +y
                (
                    [2, 5, 7],
                    Point {
                        x: point.x,
                        y: point.y + 1,
                    },
                )
            }
            2 => {
                //west = -x
                (
                    [0, 1, 2],
                    Point {
                        x: point.x - 1,
                        y: point.y,
                    },
                )
            }
            3 => {
                //east = +x
                (
                    [3, 4, 5],
                    Point {
                        x: point.x + 1,
                        y: point.y,
                    },
                )
            }
            _ => unreachable!(),
        };

        if to_check.into_iter().all(|i| !has_elf[i]) {
            return Some(sol);
        }
    }

    None
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
    fn neighbours(&self) -> [Point; 8] {
        [
            Point {
                x: self.x - 1,
                y: self.y - 1,
            },
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x - 1,
                y: self.y + 1,
            },
            Point {
                x: self.x + 1,
                y: self.y - 1,
            },
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x + 1,
                y: self.y + 1,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::DayResult;

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/23.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(110.into()),
                part2: Some(20.into()),
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/23.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(3_815.into()),
                part2: Some(893.into()),
            }
        );
    }
}
