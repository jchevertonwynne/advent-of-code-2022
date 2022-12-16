use crate::{DayResult, IntoDayResult};
use fxhash::FxBuildHasher;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;
use std::cmp::{max, min};
use std::collections::HashSet;

pub fn run(input: &'static str, is_test: bool) -> anyhow::Result<DayResult> {
    let mut input = input;
    let (row, max_x, max_y) = if is_test {
        (10, 20, 20)
    } else {
        (2_000_000, 4_000_000, 4_000_000)
    };

    let mut ranges = vec![];
    let mut sensors_and_manhattan = vec![];

    let mut beacons_on_line = HashSet::with_hasher(FxBuildHasher::default());
    while let Ok((_input, (sensor, beacon))) = parse_sensors_and_beacon(input) {
        input = _input;

        if beacon.y == row {
            beacons_on_line.insert(beacon.x);
        }

        let manhattan = sensor.manhattan(&beacon);
        let dist = (sensor.y - row).abs();
        let rem = manhattan - dist;

        sensors_and_manhattan.push((sensor, manhattan));

        if rem < 0 {
            continue;
        }

        ranges.push(sensor.x - rem..=sensor.x + rem);
    }

    let range_sum = ranges
        .iter()
        .map(|r| (r.end() - r.start()) + 1)
        .sum::<i64>();
    let overlaps = ranges
        .iter()
        .enumerate()
        .flat_map(|(i, r1)| ranges[0..i].iter().map(move |r2| (r1, r2)))
        .filter_map(|(r1, r2)| {
            if r1.start() > r2.end() || r2.start() > r1.end() {
                None
            } else {
                Some((min(r1.end(), r2.end()) - max(r1.start(), r2.start())) + 1)
            }
        })
        .sum::<i64>();

    let part_1 = range_sum - (overlaps + beacons_on_line.len() as i64);

    sensors_and_manhattan.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    let part_2 = solve_part2(&sensors_and_manhattan, max_x, max_y);

    (part_1, part_2).into_result()
}

fn solve_part2(sensors_and_manhattan: &[(Point, i64)], max_x: i64, max_y: i64) -> i64 {
    let mut x = 0;
    while x <= max_x {
        let mut y = 0;
        while y <= max_y {
            let mut ok = true;
            for &(sensor, man) in sensors_and_manhattan {
                let x_dist = (sensor.x - x).abs();
                if x_dist > man {
                    continue;
                }
                let y_dist = man - x_dist;

                let min_y = sensor.y - y_dist;
                let max_y = sensor.y + y_dist;
                if y >= min_y && y <= max_y {
                    ok = false;
                    y = max_y + 1;
                    break;
                }
            }

            if ok {
                return 4_000_000 * x + y;
            }
        }

        x += 1;
    }

    unreachable!();
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn manhattan(&self, other: &Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn parse_sensors_and_beacon(input: &str) -> IResult<&str, (Point, Point)> {
    map(
        tuple((
            preceded(tag("Sensor at x="), nom::character::complete::i64),
            preceded(tag(", y="), nom::character::complete::i64),
            preceded(
                tag(": closest beacon is at x="),
                nom::character::complete::i64,
            ),
            delimited(tag(", y="), nom::character::complete::i64, tag("\n")),
        )),
        |(x1, y1, x2, y2)| (Point { x: x1, y: y1 }, Point { x: x2, y: y2 }),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::DayResult;

    #[test]
    fn test_example_answers() {
        std::env::set_var("TEST", "1");
        let result = run(include_str!("../../input/test/15.txt"), true);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(26.into()),
                part2: Some(56_000_011.into()),
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/15.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(5_125_700.into()),
                part2: Some(11_379_394_658_764_i64.into()),
            }
        );
    }
}
