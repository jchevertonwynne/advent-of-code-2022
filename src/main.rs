use advent_of_code_2022::days;
use advent_of_code_2022::days::DayResult;
use advent_of_code_2022::{run_for_repeats, Runnable};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let days: Vec<fn() -> anyhow::Result<DayResult>> = vec![days::day01::run];

    let runnables = Runnable::load().context("failed to parse runnables")?;

    for runnable in runnables {
        match runnable {
            Runnable::Latest { repeats } => {
                run_for_repeats(days.len() as u32, &days, repeats.unwrap_or(1))?
            }
            Runnable::Single { day } => run_for_repeats(day, &days, 1)?,
            Runnable::Range { first, last } => (first..=last)
                .map(|day| run_for_repeats(day, &days, 1))
                .collect::<anyhow::Result<()>>()?,
            Runnable::Repeat { day, repeats } => run_for_repeats(day, &days, repeats)?,
        }
    }

    Ok(())
}
