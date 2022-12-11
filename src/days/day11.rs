use crate::{DayResult, IntoDayResult};
use anyhow::Context;
use std::collections::VecDeque;

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    let monkeys = load_monkeys(input)?;

    let part1 = runner(monkeys.clone(), 20, true, 0);
    let part2 = runner(
        monkeys.clone(),
        10000,
        false,
        monkeys.iter().map(|m| m.div).product(),
    );

    (part1, part2).into_result()
}

fn runner(mut monkeys: Vec<Monkey>, rounds: usize, part1: bool, modulo: usize) -> usize {
    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            let mut monkey = Monkey::default();
            std::mem::swap(&mut monkey, &mut monkeys[m]);

            for &item in &monkey.items {
                let new_score = if part1 {
                    monkey.op.apply(item) / 3
                } else {
                    monkey.op.apply(item) % modulo
                };
                if new_score % monkey.div == 0 {
                    monkeys[monkey.if_true].items.push_back(new_score);
                } else {
                    monkeys[monkey.if_false].items.push_back(new_score);
                }
            }
            monkey.inspections += monkey.items.len();
            monkey.items.clear();

            std::mem::swap(&mut monkey, &mut monkeys[m]);
        }
    }

    let mut highest = [0; 2];
    for monkey in monkeys {
        let ins = monkey.inspections;
        if ins > highest[0] {
            highest[1] = highest[0];
            highest[0] = ins;
        } else if ins > highest[1] {
            highest[1] = ins;
        }
    }

    highest[0] * highest[1]
}

fn load_monkeys(input: &str) -> anyhow::Result<Vec<Monkey>> {
    let mut monkeys = vec![];

    for monkey_text in input.split("\n\n") {
        let mut lines = monkey_text.lines();
        lines.next().context("no next line found")?;
        let starting_items = lines.next().context("no next line found")?;
        let items = starting_items[18..]
            .split(", ")
            .map(|n| n.parse())
            .collect::<Result<_, _>>()?;
        let op_line = lines.next().context("no next line found")?.as_bytes();
        let op = if op_line[23] == b'+' {
            if op_line[25] == b'o' {
                Op::Add(Val::Old)
            } else {
                Op::Add(Val::Const(
                    unsafe { std::str::from_utf8_unchecked(&op_line[25..]) }.parse()?,
                ))
            }
        } else if op_line[25] == b'o' {
            Op::Mul(Val::Old)
        } else {
            Op::Mul(Val::Const(
                unsafe { std::str::from_utf8_unchecked(&op_line[25..]) }.parse()?,
            ))
        };
        let div_line = lines.next().context("no next line found")?.as_bytes();
        let div = unsafe { std::str::from_utf8_unchecked(&div_line[21..]) }.parse()?;
        let if_true_line = lines.next().context("no next line found")?.as_bytes();
        let if_false_line = lines.next().context("no next line found")?.as_bytes();
        monkeys.push(Monkey {
            items,
            op,
            div,
            if_true: (if_true_line[29] - b'0') as usize,
            if_false: (if_false_line[30] - b'0') as usize,
            inspections: 0,
        });
    }

    Ok(monkeys)
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    op: Op,
    div: usize,
    if_true: usize,
    if_false: usize,
    inspections: usize,
}

impl Default for Monkey {
    fn default() -> Self {
        Monkey {
            items: VecDeque::new(),
            op: Op::Mul(Val::Old),
            div: 0,
            if_true: 0,
            if_false: 0,
            inspections: 0,
        }
    }
}

#[derive(Debug, Clone)]
enum Op {
    Add(Val),
    Mul(Val),
}

impl Op {
    fn apply(&self, val: usize) -> usize {
        match self {
            Op::Add(op_val) => val + op_val.val(val),
            Op::Mul(op_val) => val * op_val.val(val),
        }
    }
}

#[derive(Debug, Clone)]
enum Val {
    Old,
    Const(usize),
}

impl Val {
    fn val(&self, old: usize) -> usize {
        match self {
            Val::Old => old,
            Val::Const(c) => *c,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::DayResult;

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/11.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(10605.into()),
                part2: Some(2713310158_usize.into()),
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/11.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(54036.into()),
                part2: Some(13237873355_usize.into()),
            }
        );
    }
}
