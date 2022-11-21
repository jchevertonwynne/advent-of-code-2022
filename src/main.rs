use advent_of_code_2022::{days, DayEntry};
use advent_of_code_2022::{run_day, Runnable};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let is_test = std::env::var_os("TEST").is_some();

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

    let runnables =
        Runnable::load_all(std::env::args().skip(1)).context("failed to parse runnables")?;

    for runnable in runnables {
        match runnable {
            Runnable::Latest => {
                let day = days.len() as u32;
                run_day(day, &days[(day - 1) as usize], is_test)?;
            }
            Runnable::All => {
                let last = days.len() as u32;
                (1..=last).try_for_each(|day| run_day(day, &days[(day - 1) as usize], is_test))?;
            }
            Runnable::Range { first, last } => {
                (first..=last)
                    .try_for_each(|day| run_day(day, &days[(day - 1) as usize], is_test))?;
            }
        }
    }

    Ok(())
}
