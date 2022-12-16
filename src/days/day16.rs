use crate::{DayResult, IntoDayResult};
use fxhash::FxBuildHasher;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;
use std::cmp::max;
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

    println!("{:?}", distances);

    let worth_turning_on = valves
        .iter()
        .filter(|(_, info)| info.flow_rate != 0)
        .map(|(name, _)| *name)
        .collect::<HashSet<_, FxBuildHasher>>();

    let part_1 = solve(
        30,
        "AA",
        &valves,
        &distances,
        &worth_turning_on,
        &mut HashSet::with_hasher(FxBuildHasher::default()),
        0,
        0,
    );

    part_1.into_result()
}

#[allow(clippy::too_many_arguments)]
fn solve<'a, H: BuildHasher>(
    turn: u64,
    current_valve: &'a str,
    valves: &HashMap<&'a str, Valve, H>,
    distances: &HashMap<(&'a str, &'a str), u64, H>,
    worth_turning_on: &HashSet<&'a str, H>,
    turned_on: &mut HashSet<&'a str, H>,
    flow_rate: u64,
    cumulative: u64,
) -> u64 {
    let new_cumulative = cumulative + flow_rate;
    let mut result = new_cumulative + flow_rate * turn;

    if turn == 0 {
        return result;
    }

    if turned_on.insert(current_valve) {
        let new_flow_rate = flow_rate + valves.get(&current_valve).unwrap().flow_rate;
        result = max(
            result,
            solve(
                turn - 1,
                current_valve,
                valves,
                distances,
                worth_turning_on,
                turned_on,
                new_flow_rate,
                new_cumulative,
            ),
        );
        turned_on.remove(current_valve);
    }

    for &worth_checking in worth_turning_on {
        if worth_checking == current_valve {
            continue;
        }

        if turned_on.contains(&worth_checking) {
            continue;
        }

        let dist = *distances.get(&(current_valve, worth_checking)).unwrap();
        if dist >= turn {
            continue;
        }

        result = max(
            result,
            solve(
                turn - dist,
                worth_checking,
                valves,
                distances,
                worth_turning_on,
                turned_on,
                flow_rate,
                new_cumulative,
            ),
        );
    }

    result
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
