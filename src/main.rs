use advent_of_code_2022::{days, DayEntry};
use advent_of_code_2022::{run_day, Runnable};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let is_test = std::env::var("TEST").is_ok();

    let days = vec![
        DayEntry {
            f: days::day01::run,
            real: include_str!("../input/real/01.txt"),
            test: include_str!("../input/test/01.txt"),
        },
        DayEntry {
            f: days::day02::run,
            real: include_str!("../input/real/02.txt"),
            test: include_str!("../input/test/02.txt"),
        },
    ];

    let runnables = Runnable::load_all().context("failed to parse runnables")?;

    for runnable in runnables {
        match runnable {
            Runnable::Latest => {
                let day = days
                    .len()
                    .try_into()
                    .context("could not convert vec len to u32")?;
                run_day(day, &days, is_test)?;
            }
            Runnable::All => {
                let last = days
                    .len()
                    .try_into()
                    .context("could not convert vec len to u32")?;
                (1..=last).try_for_each(|day| run_day(day, &days, is_test))?;
            }
            Runnable::Range { first, last } => {
                (first..=last).try_for_each(|day| run_day(day, &days, is_test))?;
            }
        }
    }

    Ok(())
}
