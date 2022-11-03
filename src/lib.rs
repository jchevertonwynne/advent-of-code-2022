pub mod days;

use std::time::Instant;

use crate::days::DayResult;
use anyhow::Context;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character,
    combinator::{all_consuming, map, opt},
    sequence::{pair, preceded, separated_pair},
};
use thiserror::Error;

pub fn run_for_repeats(
    day: u32,
    days: &[fn() -> anyhow::Result<DayResult>],
    repeats: u32,
) -> anyhow::Result<()> {
    let day_fn = days
        .get((day - 1) as usize)
        .context("day index did not exist")?;

    let start = Instant::now();
    let mut answer = None;
    for _ in 0..repeats {
        answer = Some(day_fn().context("failed to run day")?)
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
    Latest {
        repeats: Option<u32>,
    }, // ! !10
    Range {
        first: u32,
        last: u32,
        repeats: Option<u32>,
    }, // 12-15 12-15:100
    Repeat {
        day: u32,
        repeats: Option<u32>,
    }, // 12 12:100
}

impl Runnable {
    pub fn load() -> anyhow::Result<Vec<Runnable>> {
        let mut runnables: Vec<Runnable> = Vec::new();
        for arg in std::env::args().skip(1) {
            runnables.push(arg.try_into().context("failed to parse runnable command")?);
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

        match res {
            Runnable::Range {
                first,
                last,
                repeats: _,
            } if first > last => {
                return Err(ConversionError::OutOfOrder);
            }
            _ => {}
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
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Parse failure: {0}")]
    ParseFailure(String),
}

impl From<nom::Err<nom::error::Error<&str>>> for ConversionError {
    fn from(err: nom::Err<nom::error::Error<&str>>) -> Self {
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
        map(parse_latest, |repeats| Runnable::Latest { repeats }),
        map(parse_range, |(first, last, repeats)| Runnable::Range {
            first,
            last,
            repeats,
        }),
        map(parse_repeat, |(day, repeats)| Runnable::Repeat {
            day,
            repeats,
        }),
    ))(input)
}

fn parse_latest(input: &str) -> nom::IResult<&str, Option<u32>> {
    all_consuming(preceded(tag("!"), opt(character::complete::u32)))(input)
}

fn parse_range(input: &str) -> nom::IResult<&str, (u32, u32, Option<u32>)> {
    all_consuming(map(
        pair(
            separated_pair(character::complete::u32, tag("-"), character::complete::u32),
            map(opt(pair(tag(":"), character::complete::u32)), |pair| {
                pair.map(|p| p.1)
            }),
        ),
        |((a, b), c)| (a, b, c),
    ))(input)
}

fn parse_repeat(input: &str) -> nom::IResult<&str, (u32, Option<u32>)> {
    all_consuming(pair(
        character::complete::u32,
        map(opt(pair(tag(":"), character::complete::u32)), |pair| {
            pair.map(|p| p.1)
        }),
    ))(input)
}
