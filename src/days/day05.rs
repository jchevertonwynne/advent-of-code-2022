use crate::{DayResult, IntoDayResult};
use anyhow::Context;
use regex::Regex;

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    let lines = input.lines().collect::<Vec<_>>();

    let mut i = 0;
    while lines[i].as_bytes()[0] == b' ' {
        i += 1;
    }
    while lines[i].as_bytes()[0] == b'[' {
        i += 1;
    }

    let commands = i + 2;

    let mut cranes_part1: Vec<Vec<char>> = Vec::new();
    for _ in (1..lines[i].len()).step_by(4) {
        cranes_part1.push(Vec::new());
    }

    while i != 0 {
        i -= 1;
        for (col, ind) in (1..lines[i].len()).step_by(4).enumerate() {
            let c = lines[i].as_bytes()[ind];
            if c.is_ascii_alphabetic() {
                cranes_part1[col].push(c as char);
            }
        }
    }

    let mut cranes_part2 = cranes_part1.clone();

    let re = Regex::new(r"move (?P<count>\d+) from (?P<start>\d+) to (?P<dest>\d+)")
        .context("failed to parse regex")?;

    let mut stack = Vec::new();

    for command in &lines[commands..] {
        let c = re.captures(command).context("failed to parse command")?;
        let count = c["count"]
            .parse::<usize>()
            .context("failed to parse count")?;
        let start = c["start"]
            .parse::<usize>()
            .context("failed to parse start")?;
        let dest = c["dest"].parse::<usize>().context("failed to parse dest")?;

        for _ in 0..count {
            let item = cranes_part1[start - 1]
                .pop()
                .context("failed to pop from cranes")?;
            cranes_part1[dest - 1].push(item);

            let item = cranes_part2[start - 1].pop().context("exp an item")?;
            stack.push(item);
        }

        for &item in stack.iter().rev() {
            cranes_part2[dest - 1].push(item);
        }
        stack.clear();
    }

    let mut part1 = String::with_capacity(cranes_part1.len());
    for crane in &cranes_part1 {
        part1.push(*crane.last().context("expected a top iten")?);
    }
    let mut part2 = String::with_capacity(cranes_part2.len());
    for crane in &cranes_part2 {
        part2.push(*crane.last().context("expected a top iten")?);
    }

    (part1, part2).into_result()
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::{DayResult};

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/05.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some("CMZ".to_string().into()),
                part2: Some("MCD".to_string().into()),
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/05.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some("CNSZFDVLJ".to_string().into()),
                part2: Some("QNDWLMGNS".to_string().into()),
            }
        );
    }
}
