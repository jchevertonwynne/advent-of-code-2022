use crate::{DayResult, IntoDayResult};
use anyhow::Context;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::is_alphabetic;
use nom::combinator::{map, map_res};
use nom::sequence::tuple;
use nom::IResult;
use std::cell::Cell;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::rc::Rc;

pub fn run(input: &'static str, _: bool) -> anyhow::Result<DayResult> {
    let mut input = input.as_bytes();

    let mut ops_optimised = HashMap::new();
    let mut to_optimise = VecDeque::new();

    while !input.is_empty() {
        let (_input, (k, op)) = parse_line(input)?;
        input = _input;
        match op {
            OpUnoptimised::Literal(v) => {
                ops_optimised.insert(k, Rc::new(Op::Literal(Cell::new(v))));
            }
            OpUnoptimised::Compound {
                first,
                sign,
                second,
            } => {
                to_optimise.push_front((k, (first, sign, second)));
            }
        }
    }

    while let Some(entry @ (name, (first, sign, second))) = to_optimise.pop_front() {
        match ops_optimised.get(first).zip(ops_optimised.get(second)) {
            None => {
                to_optimise.push_back(entry);
            }
            Some((first, second)) => {
                let first = Rc::clone(first);
                let second = Rc::clone(second);
                ops_optimised.insert(
                    name,
                    Rc::new(Op::Compound {
                        first,
                        sign,
                        second,
                    }),
                );
            }
        }
    }

    let root = ops_optimised.get("root").context("failed to find root")?;
    let part1 = root.value_of();

    let (l, r) = match root.as_ref() {
        Op::Literal(_) => unreachable!(),
        Op::Compound { first, second, .. } => (first, second),
    };

    let left = l.value_of();
    let right = r.value_of();

    let mut base = 0;

    let Op::Literal(humn) =  ops_optimised.get("humn").context("expected to find humn")?.as_ref() else {
        unreachable!();
    };

    humn.set(humn.get() - 500);
    let (_const, to_compute, signum) = if l.value_of() != left {
        (right, l, (right - left).signum())
    } else {
        (left, r, (left - right).signum())
    };

    'outer: loop {
        let mut curr = 1;
        loop {
            humn.set(base + curr);

            let new_compute = to_compute.value_of();
            let diff = (_const - new_compute) * signum;
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

    let part2 = humn.get();

    (part1, part2).into_result()
}

fn parse_line(line: &[u8]) -> IResult<&[u8], (&str, OpUnoptimised)> {
    map(
        tuple((
            map_res(take_while_m_n(4, 4, is_alphabetic), |b| {
                std::str::from_utf8(b)
            }),
            tag(b": "),
            alt((
                map(nom::character::complete::i64, OpUnoptimised::Literal),
                map(
                    tuple((
                        map_res(take_while_m_n(4, 4, is_alphabetic), |b| {
                            std::str::from_utf8(b)
                        }),
                        tag(" "),
                        alt((
                            map(tag("+"), |_| Sign::Add),
                            map(tag("-"), |_| Sign::Sub),
                            map(tag("*"), |_| Sign::Mul),
                            map(tag("/"), |_| Sign::Div),
                        )),
                        tag(b" "),
                        map_res(take_while_m_n(4, 4, is_alphabetic), |b| {
                            std::str::from_utf8(b)
                        }),
                    )),
                    |(first, _, sign, _, second)| OpUnoptimised::Compound {
                        first,
                        sign,
                        second,
                    },
                ),
            )),
            tag(b"\n"),
        )),
        |(name, _, op, _)| (name, op),
    )(line)
}

#[derive(Debug)]
enum OpUnoptimised<'a> {
    Literal(i64),
    Compound {
        first: &'a str,
        sign: Sign,
        second: &'a str,
    },
}

#[derive(Debug)]
enum Op {
    Literal(Cell<i64>),
    Compound {
        first: Rc<Op>,
        sign: Sign,
        second: Rc<Op>,
    },
}

impl Op {
    fn value_of(&self) -> i64 {
        match self {
            Op::Literal(v) => v.get(),
            Op::Compound {
                first,
                sign,
                second,
            } => {
                let first = first.value_of();
                let second = second.value_of();

                sign.apply(first, second)
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
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
                part1: Some(194_058_098_264_286_i64.into()),
                part2: Some(3_592_056_845_086_i64.into()),
            }
        );
    }
}
