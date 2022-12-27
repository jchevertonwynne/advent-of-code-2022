use crate::days::day18::SurfaceType::{Exterior, Interior, Surface};
use crate::{DayResult, IntoDayResult};
use fxhash::FxBuildHasher;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::hash::BuildHasher;

pub fn run(mut input: &'static str, _: bool) -> anyhow::Result<DayResult> {
    let mut points = HashSet::with_hasher(FxBuildHasher::default());

    while !input.is_empty() {
        let (_input, line) = parse_line(input)?;
        input = _input;
        points.insert(line);
    }

    let mut part1 = 0;
    for &point in &points {
        let mods = [
            [1, 0, 0],
            [-1, 0, 0],
            [0, 1, 0],
            [0, -1, 0],
            [0, 0, 1],
            [0, 0, -1],
        ];
        for _mod in mods {
            let mut modified = [0; 3];
            for (m, (a, b)) in modified.iter_mut().zip(point.iter().zip(_mod.iter())) {
                *m = *a + *b;
            }
            if !points.contains(&modified) {
                part1 += 1;
            }
        }
    }

    let (mins, maxes) = points.iter().fold(([i64::MAX; 3], [0; 3]), |(mi, ma), &b| {
        (
            [min(mi[0], b[0]), min(mi[1], b[1]), min(mi[2], b[2])],
            [max(ma[0], b[0]), max(ma[1], b[1]), max(ma[2], b[2])],
        )
    });

    let mut known = HashMap::with_hasher(FxBuildHasher::default());
    known.extend(points.iter().map(|&p| (p, Surface)));

    let mut visited = HashSet::new();

    let mut part2 = 0;
    for &point in &points {
        let mods = [
            [1, 0, 0],
            [-1, 0, 0],
            [0, 1, 0],
            [0, -1, 0],
            [0, 0, 1],
            [0, 0, -1],
        ];
        for _mod in mods {
            let mut modified = [0; 3];
            for (m, (a, b)) in modified.iter_mut().zip(point.iter().zip(_mod.iter())) {
                *m = *a + *b;
            }

            visited.clear();
            if let Exterior = is_exterior(modified, &mut known, &mut visited, mins, maxes) {
                part2 += 1;
            }
        }
    }

    (part1, part2).into_result()
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum SurfaceType {
    Interior,
    Exterior,
    Surface,
}

fn is_exterior(
    point: [i64; 3],
    known: &mut HashMap<[i64; 3], SurfaceType, impl BuildHasher>,
    visited: &mut HashSet<[i64; 3], impl BuildHasher>,
    mins: [i64; 3],
    maxes: [i64; 3],
) -> SurfaceType {
    if let Some(&surface_type) = known.get(&point) {
        return surface_type;
    }

    let mut rec_checker = || {
        if point[0] < mins[0] {
            return (Exterior, true);
        }
        if point[0] > maxes[0] {
            return (Exterior, true);
        }

        if point[1] < mins[1] {
            return (Exterior, true);
        }
        if point[1] > maxes[1] {
            return (Exterior, true);
        }

        if point[2] < mins[2] {
            return (Exterior, true);
        }
        if point[2] > maxes[2] {
            return (Exterior, true);
        }

        let mods = [
            [1, 0, 0],
            [-1, 0, 0],
            [0, 1, 0],
            [0, -1, 0],
            [0, 0, 1],
            [0, 0, -1],
        ];

        if mods
            .into_iter()
            .filter_map(|_mod| {
                let mut modified = [0; 3];
                for (m, (a, b)) in modified.iter_mut().zip(point.iter().zip(_mod.iter())) {
                    *m = *a + *b;
                }

                if !visited.insert(modified) {
                    return None;
                }

                Some(is_exterior(modified, known, visited, mins, maxes))
            })
            .any(|v| v == Exterior)
        {
            (Exterior, true)
        } else {
            (Interior, false)
        }
    };

    let (surface, def_known) = rec_checker();
    if def_known {
        known.insert(point, surface);
    }

    surface
}

fn parse_line(line: &str) -> IResult<&str, [i64; 3]> {
    map(
        tuple((
            nom::character::complete::i64,
            tag(","),
            nom::character::complete::i64,
            tag(","),
            nom::character::complete::i64,
            tag("\n"),
        )),
        |(a, _, b, _, c, _)| [a, b, c],
    )(line)
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::DayResult;

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/18.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(64.into()),
                part2: Some(58.into()),
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/18.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(3564.into()),
                part2: Some(2106.into()),
            }
        );
    }
}
