use advent_of_code_2022::{days, DayEntry};
use advent_of_code_2022::{run_day, Runnable};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let is_test = std::env::var_os("TEST").is_some();

    let days = get_days();

    let runnables =
        Runnable::load_all(std::env::args().skip(1)).context("failed to parse runnables")?;

    for runnable in runnables {
        let days_to_run = match runnable {
            Runnable::Latest => {
                let day = days.len() as u32;
                day..=day
            }
            Runnable::All => {
                let last = days.len() as u32;
                1..=last
            }
            Runnable::Range { first, last } => first..=last,
        };
        days_to_run
            .into_iter()
            .try_for_each(|day| run_day(day, &days[(day - 1) as usize], is_test))?;
    }

    Ok(())
}

fn get_days() -> Vec<DayEntry> {
    vec![
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
        DayEntry {
            f: days::day03::run,
            real: include_str!("../input/real/03.txt"),
            test: include_str!("../input/test/03.txt"),
        },
        DayEntry {
            f: days::day04::run,
            real: include_str!("../input/real/04.txt"),
            test: include_str!("../input/test/04.txt"),
        },
        DayEntry {
            f: days::day05::run,
            real: include_str!("../input/real/05.txt"),
            test: include_str!("../input/test/05.txt"),
        },
        DayEntry {
            f: days::day06::run,
            real: include_str!("../input/real/06.txt"),
            test: include_str!("../input/test/06.txt"),
        },
        DayEntry {
            f: days::day07::run,
            real: include_str!("../input/real/07.txt"),
            test: include_str!("../input/test/07.txt"),
        },
        DayEntry {
            f: days::day08::run,
            real: include_str!("../input/real/08.txt"),
            test: include_str!("../input/test/08.txt"),
        },
        DayEntry {
            f: days::day09::run,
            real: include_str!("../input/real/09.txt"),
            test: include_str!("../input/test/09.txt"),
        },
        DayEntry {
            f: days::day10::run,
            real: include_str!("../input/real/10.txt"),
            test: include_str!("../input/test/10.txt"),
        },
        DayEntry {
            f: days::day11::run,
            real: include_str!("../input/real/11.txt"),
            test: include_str!("../input/test/11.txt"),
        },
        DayEntry {
            f: days::day12::run,
            real: include_str!("../input/real/12.txt"),
            test: include_str!("../input/test/12.txt"),
        },
        DayEntry {
            f: days::day13::run,
            real: include_str!("../input/real/13.txt"),
            test: include_str!("../input/test/13.txt"),
        },
        DayEntry {
            f: days::day14::run,
            real: include_str!("../input/real/14.txt"),
            test: include_str!("../input/test/14.txt"),
        },
        DayEntry {
            f: days::day15::run,
            real: include_str!("../input/real/15.txt"),
            test: include_str!("../input/test/15.txt"),
        },
        DayEntry {
            f: days::day16::run,
            real: include_str!("../input/real/16.txt"),
            test: include_str!("../input/test/16.txt"),
        },
        DayEntry {
            f: days::day17::run,
            real: include_str!("../input/real/17.txt"),
            test: include_str!("../input/test/17.txt"),
        },
        DayEntry {
            f: days::day18::run,
            real: include_str!("../input/real/18.txt"),
            test: include_str!("../input/test/18.txt"),
        },
        DayEntry {
            f: days::day19::run,
            real: include_str!("../input/real/19.txt"),
            test: include_str!("../input/test/19.txt"),
        },
        DayEntry {
            f: days::day20::run,
            real: include_str!("../input/real/20.txt"),
            test: include_str!("../input/test/20.txt"),
        },
        DayEntry {
            f: days::day21::run,
            real: include_str!("../input/real/21.txt"),
            test: include_str!("../input/test/21.txt"),
        },
        DayEntry {
            f: days::day22::run,
            real: include_str!("../input/real/22.txt"),
            test: include_str!("../input/test/22.txt"),
        },
        DayEntry {
            f: days::day23::run,
            real: include_str!("../input/real/23.txt"),
            test: include_str!("../input/test/23.txt"),
        },
        DayEntry {
            f: days::day24::run,
            real: include_str!("../input/real/24.txt"),
            test: include_str!("../input/test/24.txt"),
        },
    ]
}
