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
                let day = days.len().try_into().context("failed conv")?;
                run_for_repeats(day, &days, repeats)?;
            }
            Runnable::Range {
                first,
                last,
                repeats,
            } => (first..=last).try_for_each(|day| run_for_repeats(day, &days, repeats))?,
            Runnable::Repeat { day, repeats } => run_for_repeats(day, &days, repeats)?,
        }
    }

    Ok(())
}
