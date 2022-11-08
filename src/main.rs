use advent_of_code_2022::{days, DayEntry};
use advent_of_code_2022::{run_for_repeats, Runnable};

use anyhow::Context;

const DAYS: [DayEntry; 1] = [DayEntry {
    f: days::day01::run,
    real: include_str!("../input/real/01.txt"),
    test: include_str!("../input/test/01.txt"),
}];

fn main() -> anyhow::Result<()> {
    let is_test = std::env::var("TEST").is_ok();

    println!("is test: {is_test}");

    let runnables = Runnable::load().context("failed to parse runnables")?;

    for runnable in runnables {
        match runnable {
            Runnable::Latest { repeats } => {
                let day = DAYS.len().try_into().context("failed conv")?;
                run_for_repeats(day, &DAYS, repeats, is_test)?;
            }
            Runnable::Range {
                first,
                last,
                repeats,
            } => {
                (first..=last).try_for_each(|day| run_for_repeats(day, &DAYS, repeats, is_test))?;
            }
            Runnable::Repeat { day, repeats } => run_for_repeats(day, &DAYS, repeats, is_test)?,
        }
    }

    Ok(())
}
