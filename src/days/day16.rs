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

    while let Ok((_input, (name, flow_rate, leads_to))) = parse_row(input) {
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

    let part_1 = solve(
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

    let (part_2, mut history) = solve2(
        26,
        "AA",
        "AA",
        0,
        0,
        0,
        0,
        &valves,
        &distances,
        &worth_turning_on,
        turned_on,
        0,
        0,
        vec![]
    );

    history.iter_mut().for_each(|v| v.1 = 27 - v.1);
    history.sort_unstable_by(|v1, v2| v1.1.cmp(&v2.1));
    println!("{:?}", history);

    (part_1, part_2).into_result()
}

#[allow(clippy::too_many_arguments)]
fn solve<'a, H: BuildHasher>(
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
                solve(
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

#[allow(clippy::too_many_arguments)]
fn solve2<'a, H: BuildHasher>(
    turn: u64,
    a_valve: &'a str,
    b_valve: &'a str,
    a_remaining: u64,
    b_remaining: u64,
    a_future_flow: u64,
    b_future_flow: u64,
    valves: &HashMap<&'a str, Valve, H>,
    distances: &HashMap<(&'a str, &'a str), u64, H>,
    worth_turning_on: &Vec<&'a str>,
    turned_on: &mut HashSet<&'a str, H>,
    flow_rate: u64,
    cumulative: u64,
    history: Vec<(&'a str, u64)>
) -> (u64, Vec<(&'a str, u64)>) {
    if turn == 0 {
        return (cumulative, history);
    }

    let mut result = cumulative + turn * flow_rate;
    let mut best_history = history.clone();

    if a_remaining == 0 {
        for &new_a_valve in worth_turning_on {
            if new_a_valve == a_valve {
                continue;
            }

            let dist = *distances.get(&(a_valve, new_a_valve)).unwrap();
            if dist >= turn {
                continue;
            }

            if turned_on.insert(new_a_valve) {
                let new_valve = valves.get(&new_a_valve).unwrap();
                let wanted_dist = dist + 1;
                let actual_dist = min(wanted_dist, b_remaining);
                let new_flow_rate = flow_rate + a_future_flow;
                let new_cumulative = cumulative + new_flow_rate * actual_dist;
                let new_a_future_flow = new_valve.flow_rate;
                let new_turn = turn - actual_dist;
                let mut new_history = history.clone();
                new_history.push((new_a_valve, turn - wanted_dist));
                let (a, new_history) = solve2(
                    new_turn,
                    new_a_valve,
                    b_valve,
                    wanted_dist - actual_dist,
                    b_remaining - actual_dist,
                    new_a_future_flow,
                    b_future_flow,
                    valves,
                    distances,
                    worth_turning_on,
                    turned_on,
                    new_flow_rate,
                    new_cumulative,
                    new_history,
                );
                if a > result {
                    result = a;
                    best_history = new_history;
                }

                turned_on.remove(&new_a_valve);
            }
        }
    }

    if b_remaining == 0 {
        for &new_b_valve in worth_turning_on {
            if new_b_valve == b_valve {
                continue;
            }

            let dist = *distances.get(&(b_valve, new_b_valve)).unwrap();
            if dist >= turn {
                continue;
            }

            if turned_on.insert(new_b_valve) {
                let new_valve = valves.get(&new_b_valve).unwrap();
                let wanted_dist = dist + 1;
                let actual_dist = min(wanted_dist, a_remaining);
                let new_flow_rate = flow_rate + b_future_flow;
                let new_cumulative = cumulative + new_flow_rate * actual_dist;
                let new_b_future_flow = new_valve.flow_rate;
                let new_turn = turn - actual_dist;
                let mut new_history = history.clone();
                new_history.push((new_b_valve, turn));
                let (a, new_history) = solve2(
                    new_turn,
                    a_valve,
                    new_b_valve,
                    a_remaining - actual_dist,
                    wanted_dist - actual_dist,
                    a_future_flow,
                    new_b_future_flow,
                    valves,
                    distances,
                    worth_turning_on,
                    turned_on,
                    new_flow_rate,
                    new_cumulative,
                    new_history
                );
                if a > result {
                    result = a;
                    best_history = new_history;
                }

                turned_on.remove(&new_b_valve);
            }
        }
    }

    (result, best_history)
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

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/16.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: None,
                part2: None,
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/16.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: None,
                part2: None,
            }
        );
    }
}
