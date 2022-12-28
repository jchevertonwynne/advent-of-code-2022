use crate::{DayResult, IntoDayResult};
use fxhash::FxBuildHasher;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::BuildHasher;

pub fn run(mut input: &'static str, _: bool) -> anyhow::Result<DayResult> {
    let mut valves = HashMap::with_hasher(FxBuildHasher::default());

    while !input.is_empty() {
        let (_input, (name, flow_rate, leads_to)) = parse_row(input)?;
        input = _input;
        valves.insert(
            name,
            Valve {
                flow_rate,
                leads_to,
            },
        );
    }

    let mut distances = HashMap::with_hasher(FxBuildHasher::default());
    for &v1 in valves.keys() {
        for &v2 in valves.keys() {
            let dist = dist_to(v1, v2, &valves);
            distances.insert((v1, v2), dist);
            distances.insert((v2, v1), dist);
        }
    }

    let worth_turning_on = valves
        .iter()
        .filter(|(_, info)| info.flow_rate != 0)
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();

    let turned_on = &mut HashSet::with_hasher(FxBuildHasher::default());

    let part_1 = solve_part1(
        30,
        "AA",
        &valves,
        &distances,
        &worth_turning_on,
        turned_on,
        0,
        0,
    );

    turned_on.clear();

    let start = Journey {
        valve: "AA",
        remaining_turns: 0,
        future_flow: 0,
    };
    let part_2 = solve_part2(
        26,
        0,
        0,
        start.clone(),
        start.clone(),
        false,
        &valves,
        &distances,
        &worth_turning_on,
        turned_on,
    );

    (part_1, part_2).into_result()
}

#[allow(clippy::too_many_arguments)]
fn solve_part2<'a, H: BuildHasher>(
    turns_remaining: u64,
    flow_rate: u64,
    cumulative: u64,
    a: Journey,
    b: Journey,
    flipped: bool,
    valves: &HashMap<&'a str, Valve, H>,
    distances: &HashMap<(&'a str, &'a str), u64, H>,
    worth_turning_on: &[&'a str],
    turned_on: &mut HashSet<&'a str, H>,
) -> u64 {
    if turns_remaining == 0 {
        return cumulative;
    }

    let mut result = cumulative + turns_remaining * flow_rate;

    let Journey {
        valve: a_valve,
        remaining_turns: a_remaining_turns,
        future_flow: a_future_flow,
    } = a;

    if a_remaining_turns == 0 {
        let new_flow_rate = flow_rate + a_future_flow;

        if worth_turning_on.len() != turned_on.len() {
            for &new_a_valve in worth_turning_on {
                if new_a_valve == a_valve {
                    continue;
                }

                let dist = *distances.get(&(a_valve, new_a_valve)).unwrap();
                if dist >= turns_remaining {
                    continue;
                }

                if turned_on.insert(new_a_valve) {
                    let new_a_valve_info = valves.get(&new_a_valve).unwrap();
                    let new_a_rem_dist = dist + 1;
                    let b = b.clone();
                    let b_rem_dist = b.remaining_turns;
                    let dist_to_move = min(turns_remaining, min(new_a_rem_dist, b_rem_dist));

                    let new_cumulative = cumulative + new_flow_rate * dist_to_move;
                    let new_a_future_flow = new_a_valve_info.flow_rate;
                    let new_turns_remaining = turns_remaining - dist_to_move;

                    let new_a = Journey {
                        valve: new_a_valve,
                        remaining_turns: new_a_rem_dist - dist_to_move,
                        future_flow: new_a_future_flow,
                    };
                    let new_b = Journey {
                        valve: b.valve,
                        remaining_turns: b.remaining_turns - dist_to_move,
                        future_flow: b.future_flow,
                    };

                    let new_res = solve_part2(
                        new_turns_remaining,
                        new_flow_rate,
                        new_cumulative,
                        new_a,
                        new_b,
                        false,
                        valves,
                        distances,
                        worth_turning_on,
                        turned_on,
                    );
                    if new_res > result {
                        result = new_res;
                    }

                    turned_on.remove(new_a_valve);
                }
            }
        } else {
            let Journey {
                remaining_turns,
                future_flow,
                ..
            } = b;
            let turns_to_do = min(turns_remaining, remaining_turns);
            let new_result = cumulative
                + new_flow_rate * remaining_turns
                + (new_flow_rate + future_flow) * (turns_remaining - turns_to_do);
            if new_result > result {
                result = new_result;
            }
        }
    }

    if b.remaining_turns == 0 && !flipped {
        let new_res = solve_part2(
            turns_remaining,
            flow_rate,
            cumulative,
            b,
            a,
            true,
            valves,
            distances,
            worth_turning_on,
            turned_on,
        );

        if new_res > result {
            result = new_res;
        }
    }

    result
}

#[allow(clippy::too_many_arguments)]
fn solve_part1<'a, H: BuildHasher>(
    turn: u64,
    current_valve: &'a str,
    valves: &HashMap<&'a str, Valve, H>,
    distances: &HashMap<(&'a str, &'a str), u64, H>,
    worth_turning_on: &Vec<&'a str>,
    turned_on: &mut HashSet<&'a str, H>,
    flow_rate: u64,
    cumulative: u64,
) -> u64 {
    if turn == 0 {
        return cumulative;
    }

    let mut result = cumulative + turn * flow_rate;

    for &worth_checking in worth_turning_on {
        if worth_checking == current_valve {
            continue;
        }

        let dist = *distances.get(&(current_valve, worth_checking)).unwrap();
        if dist >= turn {
            continue;
        }

        if turned_on.insert(worth_checking) {
            let new_flow_rate = flow_rate + valves.get(&worth_checking).unwrap().flow_rate;
            let new_cumulative = cumulative + flow_rate * (dist + 1);
            result = max(
                result,
                solve_part1(
                    turn - (dist + 1),
                    worth_checking,
                    valves,
                    distances,
                    worth_turning_on,
                    turned_on,
                    new_flow_rate,
                    new_cumulative,
                ),
            );

            turned_on.remove(&worth_checking);
        }
    }

    result
}

#[derive(Clone)]
struct Journey<'a> {
    valve: &'a str,
    remaining_turns: u64,
    future_flow: u64,
}

fn dist_to(start: &str, end: &str, valves: &HashMap<&str, Valve, impl BuildHasher>) -> u64 {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    let mut seen = HashSet::new();

    while let Some((state, dist)) = queue.pop_front() {
        if !seen.insert(state) {
            continue;
        }

        if state == end {
            return dist;
        }

        let neighbours = valves
            .get(state)
            .map(|v| v.leads_to.iter().map(|&s| (s, dist + 1)))
            .unwrap();

        queue.extend(neighbours);
    }

    unreachable!();
}

#[derive(Debug)]
struct Valve<'a> {
    flow_rate: u64,
    leads_to: Vec<&'a str>,
}

fn parse_row(input: &str) -> IResult<&str, (&str, u64, Vec<&str>)> {
    tuple((
        preceded(tag("Valve "), alpha1),
        preceded(tag(" has flow rate="), nom::character::complete::u64),
        delimited(
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list1(tag(", "), alpha1),
            tag("\n"),
        ),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::DayResult;
    use std::assert_eq;

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/16.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(1_651.into()),
                part2: Some(1_707.into()),
            }
        );
    }

    // #[test]
    // fn test_answers() {
    //     let result = run(include_str!("../../input/real/16.txt"), false);
    //     assert_eq!(
    //         result.unwrap(),
    //         DayResult {
    //             part1: Some(1728.into()),
    //             part2: Some(2304.into()),
    //         }
    //     );
    // }
}
