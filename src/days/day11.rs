use crate::{DayResult, IntoDayResult};
use anyhow::Context;
use std::rc::Rc;

pub fn run(input: &'static str, _: bool) -> anyhow::Result<DayResult> {
    let monkeys = load_monkeys(input)?;

    let part1 = play_game::<true>(monkeys.clone(), 20, 0);
    let modulo = monkeys.iter().map(|m| m.div).product();
    let part2 = play_game::<false>(monkeys, 10000, modulo);

    (part1, part2).into_result()
}

fn play_game<const PART1: bool>(mut monkeys: Vec<Monkey>, rounds: usize, modulo: usize) -> usize {
    let mut monkey: Monkey = Monkey::default();
    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            std::mem::swap(&mut monkey, &mut monkeys[m]);

            monkey.inspections += monkey.items.len();
            for item in monkey.items.drain(..) {
                let val = (monkey.op)(item);
                let new_score = if PART1 { val / 3 } else { val % modulo };
                monkeys[monkey.indices[(new_score % monkey.div == 0) as usize]]
                    .items
                    .push(new_score);
            }

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
        let op: Rc<dyn Fn(usize) -> usize> = if op_line[23] == b'+' {
            if op_line[25] == b'o' {
                Rc::new(|val| val + val)
            } else {
                let int =
                    unsafe { std::str::from_utf8_unchecked(&op_line[25..]) }.parse::<usize>()?;
                Rc::new(move |val| val + int)
            }
        } else if op_line[25] == b'o' {
            Rc::new(|val| val * val)
        } else {
            let int = unsafe { std::str::from_utf8_unchecked(&op_line[25..]) }.parse::<usize>()?;
            Rc::new(move |val| val * int)
        };
        let div_line = lines.next().context("no next line found")?.as_bytes();
        let div = unsafe { std::str::from_utf8_unchecked(&div_line[21..]) }.parse()?;
        let if_true_line = lines.next().context("no next line found")?.as_bytes();
        let if_false_line = lines.next().context("no next line found")?.as_bytes();
        let indices = [
            (if_false_line[30] - b'0') as usize,
            (if_true_line[29] - b'0') as usize,
        ];
        monkeys.push(Monkey {
            items,
            op,
            div,
            indices,
            inspections: 0,
        });
    }

    Ok(monkeys)
}

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    op: Rc<dyn Fn(usize) -> usize>,
    div: usize,
    indices: [usize; 2],
    inspections: usize,
}

impl Default for Monkey {
    fn default() -> Self {
        Monkey {
            items: Vec::new(),
            op: Rc::new(|a| a + a),
            div: 0,
            indices: [0; 2],
            inspections: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::DayResult;

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/11.txt"), false);
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
        let result = run(include_str!("../../input/real/11.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(54036.into()),
                part2: Some(13237873355_usize.into()),
            }
        );
    }
}
