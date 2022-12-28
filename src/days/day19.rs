use crate::{DayResult, IntoDayResult};
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;
use std::cmp::max;
use std::collections::HashMap;
use std::hash::Hash;

pub fn run(mut input: &'static str, _: bool) -> anyhow::Result<DayResult> {
    let mut part1 = 0;
    while !input.is_empty() {
        let (_input, blueprint) = parse_line(input)?;
        input = _input;

        let score = play(14, State::default(), &blueprint, &mut HashMap::new());
        println!("{blueprint:?} has score {score}");
        part1 += blueprint.number * score
    }

    part1.into_result()
}

fn play(
    turns_remaining: usize,
    mut state: State,
    blueprint: &Blueprint,
    cache: &mut HashMap<(State, usize), i16>,
) -> i16 {
    state.materials.ore += state.robots.ore;
    state.materials.clay += state.robots.clay;
    state.materials.obsidian += state.robots.obsidian;
    state.materials.geode += state.robots.geode;

    if turns_remaining == 0 {
        return state.materials.geode;
    }

    if let Some(score) = cache.get(&(state, turns_remaining)) {
        return *score;
    }

    let mut best = state.materials.geode;

    if blueprint.ore.ore < state.materials.ore {
        let mut new_state = state;
        new_state.materials.ore -= blueprint.ore.ore;
        new_state.robots.ore += 1;
        best = max(best, play(turns_remaining - 1, new_state, blueprint, cache));
    }

    if blueprint.clay.ore < state.materials.ore {
        let mut new_state = state;
        new_state.materials.ore -= blueprint.clay.ore;
        new_state.robots.clay += 1;
        best = max(best, play(turns_remaining - 1, new_state, blueprint, cache));
    }

    if blueprint.obsidian.ore < state.materials.ore
        && blueprint.obsidian.clay < state.materials.clay
    {
        let mut new_state = state;
        new_state.materials.ore -= blueprint.obsidian.ore;
        new_state.materials.clay -= blueprint.obsidian.clay;
        new_state.robots.obsidian += 1;
        best = max(best, play(turns_remaining - 1, new_state, blueprint, cache));
    }

    if blueprint.geode.ore < state.materials.ore && blueprint.geode.obsidian < state.materials.clay
    {
        let mut new_state = state;
        new_state.materials.ore -= blueprint.geode.ore;
        new_state.materials.obsidian -= blueprint.geode.obsidian;
        new_state.robots.geode += 1;
        best = max(best, play(turns_remaining - 1, new_state, blueprint, cache));
    }

    best = max(best, play(turns_remaining - 1, state, blueprint, cache));

    cache.insert((state, turns_remaining), best);

    best
}

impl Default for State {
    fn default() -> Self {
        State {
            materials: Materials {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            robots: Robots {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct State {
    materials: Materials,
    robots: Robots,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Materials {
    ore: i16,
    clay: i16,
    obsidian: i16,
    geode: i16,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Robots {
    ore: i16,
    clay: i16,
    obsidian: i16,
    geode: i16,
}

fn parse_line(line: &str) -> IResult<&str, Blueprint> {
    map(
        tuple((
            tag("Blueprint "),
            nom::character::complete::i16,
            tag(": Each ore robot costs "),
            nom::character::complete::i16,
            tag(" ore. Each clay robot costs "),
            nom::character::complete::i16,
            tag(" ore. Each obsidian robot costs "),
            nom::character::complete::i16,
            tag(" ore and "),
            nom::character::complete::i16,
            tag(" clay. Each geode robot costs "),
            nom::character::complete::i16,
            tag(" ore and "),
            nom::character::complete::i16,
            tag(" obsidian.\n"),
        )),
        |(
            _,
            number,
            _,
            ore_ore,
            _,
            clay_ore,
            _,
            obsidian_ore,
            _,
            obsidian_clay,
            _,
            geode_ore,
            _,
            geode_obsidian,
            _,
        )| Blueprint {
            number,
            ore: OreCost { ore: ore_ore },
            clay: ClayCost { ore: clay_ore },
            obsidian: ObsidianCost {
                ore: obsidian_ore,
                clay: obsidian_clay,
            },
            geode: GeodeCost {
                ore: geode_ore,
                obsidian: geode_obsidian,
            },
        },
    )(line)
}

#[derive(Debug, Default, Copy, Clone)]
struct Blueprint {
    number: i16,
    ore: OreCost,
    clay: ClayCost,
    obsidian: ObsidianCost,
    geode: GeodeCost,
}

#[derive(Debug, Default, Copy, Clone)]
struct OreCost {
    ore: i16,
}

#[derive(Debug, Default, Copy, Clone)]
struct ClayCost {
    ore: i16,
}

#[derive(Debug, Default, Copy, Clone)]
struct ObsidianCost {
    ore: i16,
    clay: i16,
}

#[derive(Debug, Default, Copy, Clone)]
struct GeodeCost {
    ore: i16,
    obsidian: i16,
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::DayResult;

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/19.txt"), false);
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
        let result = run(include_str!("../../input/real/19.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: None,
                part2: None,
            }
        );
    }
}
