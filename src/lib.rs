pub mod days;

use std::fmt::Display;
use std::time::Instant;

use anyhow::Context;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete as character,
    combinator::{all_consuming, map},
    sequence::{pair, preceded},
};
use thiserror::Error;

pub enum Answers {
    String(String),
    Int(u64),
}

impl Display for Answers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Answers::String(s) => write!(f, "{}", s),
            Answers::Int(i) => write!(f, "{}", i),
        }
    }
}

pub struct DayResult {
    pub part1: Option<Answers>,
    pub part2: Option<Answers>,
}

pub struct DayEntry {
    pub f: fn(&'static str) -> anyhow::Result<DayResult>,
    pub real: &'static str,
    pub test: &'static str,
}

pub fn run_day(day: u32, days: &[DayEntry], is_test: bool) -> anyhow::Result<()> {
    let DayEntry { f, real, test } = days
        .get((day - 1) as usize)
        .context("day index did not exist")?;

    let input = if is_test { *test } else { *real };

    let start = Instant::now();
    let answer = f(input).context("failed to run day")?;
    let end = start.elapsed();

    println!("day {}:", day);

    if let Some(part1) = answer.part1 {
        println!("part1:");
        println!("\t{}", part1);
    }

    if let Some(part2) = answer.part2 {
        println!("part2:");
        println!("\t{}", part2);
    }

    println!("Duration:");
    println!("\t{} s", end.as_secs());
    println!("\t{} ms", end.as_millis());
    println!("\t{} us", end.as_micros());
    println!("\t{} ns", end.as_nanos());

    Ok(())
}

#[derive(Debug)]
pub enum Runnable {
    Latest,                          // .
    Range { first: u32, last: u32 }, // 12-15
}

impl Runnable {
    pub fn load_all() -> anyhow::Result<Vec<Runnable>> {
        let mut runnables: Vec<Runnable> = Vec::new();
        for arg in std::env::args().skip(1) {
            runnables.push(arg.try_into().context("failed to parse runnable command")?);
        }
        if runnables.is_empty() {
            runnables.push(Runnable::Latest);
        }
        Ok(runnables)
    }
}

impl TryFrom<String> for Runnable {
    type Error = ConversionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl<'a> TryFrom<&'a str> for Runnable {
    type Error = ConversionError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let res = parse_runnable(value).map(|r| r.1)?;

        if let Runnable::Range { first, last } = res {
            if first < last {
                return Err(ConversionError::OutOfOrder);
            }
        }

        Ok(res)
    }
}

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("Input was incomplete")]
    Incomplete,
    #[error("Day range was not increasing")]
    OutOfOrder,
    #[error("Repeats must be one or")]
    ZeroRepeats,
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Parse failure: {0}")]
    ParseFailure(String),
}

impl<T> From<nom::Err<nom::error::Error<T>>> for ConversionError {
    fn from(err: nom::Err<nom::error::Error<T>>) -> Self {
        match err {
            nom::Err::Incomplete(_) => ConversionError::Incomplete,
            nom::Err::Error(error) => ConversionError::ParseError(format!("{:?}", error.code)),
            nom::Err::Failure(failure) => {
                ConversionError::ParseFailure(format!("{:?}", failure.code))
            }
        }
    }
}

fn parse_runnable(input: &str) -> nom::IResult<&str, Runnable> {
    alt((
        map(parse_latest, |_| Runnable::Latest),
        map(parse_range, |(first, last)| Runnable::Range { first, last }),
    ))(input)
}

fn parse_latest(input: &str) -> nom::IResult<&str, &str> {
    all_consuming(tag("."))(input)
}

fn parse_range(input: &str) -> nom::IResult<&str, (u32, u32)> {
    all_consuming(pair(character::u32, preceded(tag("-"), character::u32)))(input)
}
