use anyhow::Context;
use nom::bytes::complete::tag;
use nom::character::complete as character;
use nom::combinator::all_consuming;
use nom::multi::many0;
use nom::sequence::terminated;
use nom::IResult;

use crate::days::Answers;
use crate::days::DayResult;

const INPUT: &str = include_str!("../../input/01.txt");

pub fn run() -> anyhow::Result<DayResult> {
    let (_, nums) = parse_list(INPUT).context("failed to parse input")?;
    
    Ok(DayResult {
        part1: Some(Answers::Int(nums.iter().sum())),
        part2: Some(Answers::Int(nums.iter().product())),
    })
}

fn parse_list(input: &str) -> IResult<&str, Vec<u64>> {
    all_consuming(many0(terminated(character::u64, tag("\n"))))(input)
}
