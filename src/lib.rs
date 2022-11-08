pub mod days;

use std::{time::Instant, env::Args};

use anyhow::Context;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete as character,
    combinator::{all_consuming, map, opt},
    sequence::{pair, preceded, tuple},
};
use thiserror::Error;

use crate::days::DayResult;

pub struct DayEntry {
    pub f: fn(&'static str) -> anyhow::Result<DayResult>,
    pub real: &'static str,
    pub test: &'static str,
}

pub fn run_for_repeats(
    day: u32,
    days: &[DayEntry],
    repeats: u32,
    is_test: bool,
) -> anyhow::Result<()> {
    let DayEntry { f, real, test } = days
        .get((day - 1) as usize)
        .context("day index did not exist")?;

    let input = if is_test { *test } else { *real };

    let start = Instant::now();
    let mut answer = None;
    for _ in 0..repeats {
        answer = Some(f(input).context("failed to run day")?);
    }
    let end = start.elapsed() / repeats;

    let answer = answer.context("day was not ran")?;

    println!("day {}:", day);

    if let Some(part1) = answer.part1 {
        println!("part1:");
        println!("\t{}", part1);
    }

    if let Some(part2) = answer.part2 {
        println!("part2:");
        println!("\t{}", part2);
    }

    println!("Duration: (avg of {} runs)", repeats);
    println!("\t{} s", end.as_secs());
    println!("\t{} ms", end.as_millis());
    println!("\t{} us", end.as_micros());
    println!("\t{} ns", end.as_nanos());

    Ok(())
}

#[derive(Debug)]
pub enum Runnable {
    Latest { repeats: u32 },                       // . .10
    Range { first: u32, last: u32, repeats: u32 }, // 12-15 12-15:100
    Repeat { day: u32, repeats: u32 },             // 12 12:100
}

impl Runnable {
    pub fn load() -> anyhow::Result<Vec<Runnable>> {
        let mut runnables: Vec<Runnable> = Vec::new();
        for arg in std::env::args().skip(1) {
            runnables.push(arg.try_into().context("failed to parse runnable command")?);
        }
        if runnables.is_empty() {
            runnables.push(Runnable::Latest { repeats: 1 });
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

        if let Runnable::Range {
            first,
            last,
            repeats,
        } = res
        {
            if first < last {
                return Err(ConversionError::OutOfOrder);
            }
            if repeats == 0 {
                return Err(ConversionError::ZeroRepeats);
            }
        }

        if let Runnable::Repeat { repeats: 0, .. } = res {
            return Err(ConversionError::ZeroRepeats);
        }

        if let Runnable::Range { repeats: 0, .. } = res {
            return Err(ConversionError::ZeroRepeats);
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
        map(parse_latest, |repeats| Runnable::Latest {
            repeats: repeats.unwrap_or(1),
        }),
        map(parse_range, |(first, last, repeats)| Runnable::Range {
            first,
            last,
            repeats: repeats.unwrap_or(1),
        }),
        map(parse_repeat, |(day, repeats)| Runnable::Repeat {
            day,
            repeats: repeats.unwrap_or(1),
        }),
    ))(input)
}

fn parse_latest(input: &str) -> nom::IResult<&str, Option<u32>> {
    all_consuming(preceded(tag("."), opt(character::u32)))(input)
}

fn parse_range(input: &str) -> nom::IResult<&str, (u32, u32, Option<u32>)> {
    all_consuming(tuple((
        character::u32,
        preceded(tag("-"), character::u32),
        opt(preceded(tag(":"), character::u32)),
    )))(input)
}

fn parse_repeat(input: &str) -> nom::IResult<&str, (u32, Option<u32>)> {
    all_consuming(pair(
        character::u32,
        opt(preceded(tag(":"), character::u32)),
    ))(input)
}
