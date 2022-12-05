use crate::{DayResult, IntoDayResult};
use anyhow::Context;
use nom::bytes::complete::tag;
use nom::sequence::{preceded, tuple};
use nom::IResult;
use std::collections::VecDeque;

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    let mut lines_iter = input.lines();

    let mut cranes_part1: Vec<VecDeque<char>> = vec![];

    for line in &mut lines_iter {
        let line = line.as_bytes();
        let f = line[1];
        if f != b' ' && !(b'A'..=b'Z').contains(&f) {
            break;
        }
        cranes_part1.resize_with((1..line.len()).step_by(4).count(), VecDeque::new);
        for (col, &c) in line.iter().skip(1).step_by(4).enumerate() {
            if c.is_ascii_alphabetic() {
                cranes_part1[col].push_front(c as char);
            }
        }
    }

    let mut cranes_part2 = cranes_part1.clone();

    for command in lines_iter.skip(1) {
        let (count, start, dest) = parse_line(command)?.1;

        for pos in 0..count {
            let item = cranes_part1[(start - 1) as usize]
                .pop_back()
                .context("failed to pop from cranes")?;
            cranes_part1[(dest - 1) as usize].push_back(item);

            let item = cranes_part2[(start - 1) as usize]
                [cranes_part2[(start - 1) as usize].len() - (count - pos) as usize];
            cranes_part2[(dest - 1) as usize].push_back(item);
        }

        for _ in 0..count {
            cranes_part2[(start - 1) as usize].pop_back();
        }
    }

    let part1 = cranes_part1
        .iter()
        .map(|c| c.back())
        .collect::<Option<String>>()
        .context("a crane had no contents")?;

    let part2 = cranes_part2
        .iter()
        .map(|c| c.back())
        .collect::<Option<String>>()
        .context("a crane had no contents")?;

    (part1, part2).into_result()
}

fn parse_line(line: &str) -> IResult<&str, (u32, u32, u32)> {
    tuple((
        preceded(tag("move "), nom::character::complete::u32),
        preceded(tag(" from "), nom::character::complete::u32),
        preceded(tag(" to "), nom::character::complete::u32),
    ))(line)
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::DayResult;

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/05.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some("CMZ".into()),
                part2: Some("MCD".into()),
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/05.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some("CNSZFDVLJ".into()),
                part2: Some("QNDWLMGNS".into()),
            }
        );
    }
}
