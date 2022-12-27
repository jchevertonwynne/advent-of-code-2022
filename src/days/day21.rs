use crate::{DayResult, IntoDayResult};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashMap;
use std::fmt::Debug;
use anyhow::Context;

pub fn run(input: &'static str, _: bool) -> anyhow::Result<DayResult> {
    let mut ops = HashMap::new();

    let mut input = input.as_bytes();

    while !input.is_empty() {
        let (_input, (k, op)) = parse_line(input)?;
        input = _input;
        ops.insert(k, op);
    }

    let root = ops.get("root").context("failed to find root")?;
    let part1 = root.value_of(&ops);

    let (l, r) = match root {
        Op::Literal(_) => unreachable!(),
        Op::Compound { first, second, .. } => (first, second)
    };

    let left = ops.get(l).context("failed to find left")?.value_of(&ops);
    let right = ops.get(r).context("failed to find right")?.value_of(&ops);
    let base_diff = left - right;

    let mut base = 0;

    'outer: loop {
        let mut curr = 1;
        loop {
            let humn = ops.get_mut("humn").context("expected to find humn")?;
            match humn {
                Op::Literal(l) => {
                    *l = base + curr;
                }
                Op::Compound { .. } => unreachable!(),
            }

            let root = ops.get("root").context("failed to find root")?;
            let (l, r) = match root {
                Op::Literal(_) => unreachable!(),
                Op::Compound { first, second, .. } => (first, second)
            };

            let left = ops.get(l).context("failed to find left")?.value_of(&ops);
            let right = ops.get(r).context("failed to find right")?.value_of(&ops);
            let diff = (left - right) * base_diff.signum();
            if diff == 0 {
                break 'outer;
            }

            if diff < 0 {
                break;
            }

            curr <<= 1;
        }

        base += curr >> 1;
    }

    let humn = ops.get("humn").context("expected to find humn")?;
    let part2 = match humn {
        Op::Literal(v) => *v,
        Op::Compound { .. } => unreachable!(),
    };

    (part1, part2).into_result()
}

fn parse_line(line: &[u8]) -> IResult<&[u8], (&str, Op)> {
    map(
        tuple((
            map(
                take_while_m_n(4, 4, nom::character::is_alphabetic),
                |b| unsafe { std::str::from_utf8_unchecked(b) },
            ),
            tag(b": "),
            alt((
                map(nom::character::complete::i64, Op::Literal),
                map(
                    tuple((
                        take_while_m_n(4, 4, nom::character::is_alphabetic),
                        tag(" "),
                        map(
                            alt((tag(b"+"), tag(b"-"), tag(b"*"), tag(b"/"))),
                            |s: &[u8]| match s {
                                b"+" => Sign::Add,
                                b"-" => Sign::Sub,
                                b"*" => Sign::Mul,
                                b"/" => Sign::Div,
                                _ => unreachable!(),
                            },
                        ),
                        tag(b" "),
                        take_while_m_n(4, 4, nom::character::is_alphabetic),
                    )),
                    |(first, _, sign, _, second)| {
                        let first = unsafe { std::str::from_utf8_unchecked(first) };
                        let second = unsafe { std::str::from_utf8_unchecked(second) };
                        Op::Compound {
                            first,
                            sign,
                            second,
                        }
                    },
                ),
            )),
            tag(b"\n"),
        )),
        |(name, _, op, _)| (name, op),
    )(line)
}

#[derive(Debug)]
enum Op<'a> {
    Literal(i64),
    Compound {
        first: &'a str,
        sign: Sign,
        second: &'a str,
    },
}

impl Op<'_> {
    fn value_of(&self, source: &HashMap<&str, Op>) -> i64 {
        match self {
            Op::Literal(v) => *v,
            Op::Compound {
                first,
                sign,
                second,
            } => {
                let first = source.get(first).unwrap().value_of(source);
                let second = source.get(second).unwrap().value_of(source);

                sign.apply(first, second)
            }
        }
    }
}

#[derive(Debug)]
enum Sign {
    Add,
    Sub,
    Mul,
    Div,
}

impl Sign {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Sign::Add => a + b,
            Sign::Sub => a - b,
            Sign::Mul => a * b,
            Sign::Div => a / b,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::DayResult;

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/21.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(152.into()),
                part2: Some(301.into()),
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/21.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(194058098264286_i64.into()),
                part2: Some(3592056845086_i64.into()),
            }
        );
    }
}
