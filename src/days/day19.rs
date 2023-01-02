use crate::{DayResult, IntoDayResult};
use arrayvec::ArrayVec;
use nom::bytes::complete::tag;
use nom::character::complete::u32 as nom_u32;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;
use num::Integer;
use std::cmp::max;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{AddAssign, Sub, SubAssign};

pub fn run(mut input: &'static str, _: bool) -> anyhow::Result<DayResult> {
    let mut seen = 0;
    let mut part1 = 0;
    let mut part2 = 1;
    while !input.is_empty() {
        let (_input, blueprint) = Blueprint::parse_line(input)?;
        input = _input;

        part1 += blueprint.number * State::default().best_score_from_state(blueprint);
        if seen < 3 {
            part2 *= State::part2().best_score_from_state(blueprint);
            seen += 1;
        }
    }

    (part1, part2).into_result()
}

impl State {
    fn part2() -> State {
        State { turns_remaining: 32, ..Default::default() }
    }

    fn best_score_from_state(self, blueprint: Blueprint) -> u32 {
        let mut best = self.materials.geode + self.robots.geode_robots * self.turns_remaining;
        for poss in self.next_states(blueprint) {
            best = max(best, poss.best_score_from_state(blueprint));
        }
        best
    }

    fn next_states(self, blueprint: Blueprint) -> ArrayVec<State, 4> {
        let mut res = ArrayVec::new();

        let max_ore = max(
            max(blueprint.ore_robot.ore, blueprint.clay_robot.ore),
            max(blueprint.obsidian_robot.ore, blueprint.geode_robot.ore),
        );
        if self.robots.ore_robots < max_ore {
            let turns = blueprint.time_to_make_ore_robot(self);
            if turns  < self.turns_remaining {
                let mut new_state = self - Turns(turns);
                new_state.robots.ore_robots += 1;
                new_state.materials -= blueprint.ore_robot;
                res.push(new_state);
            }
        }

        if self.robots.clay_robots < blueprint.obsidian_robot.clay {
            let turns = blueprint.time_to_make_clay_robot(self);
            if turns  < self.turns_remaining {
                let mut new_state = self - Turns(turns);
                new_state.robots.clay_robots += 1;
                new_state.materials -= blueprint.clay_robot;
                res.push(new_state);
            }
        }

        if self.robots.clay_robots != 0
            && self.robots.obsidian_robots < blueprint.geode_robot.obsidian
        {
            let turns = blueprint.time_to_make_obsidian_robot(self);
            if turns  < self.turns_remaining {
                let mut new_state = self - Turns(turns);
                new_state.robots.obsidian_robots += 1;
                new_state.materials -= blueprint.obsidian_robot;
                res.push(new_state);
            }
        }

        if self.robots.obsidian_robots != 0 {
            let turns = blueprint.time_to_make_geode_robot(self);
            if turns  < self.turns_remaining {
                let mut new_state = self - Turns(turns);
                new_state.robots.geode_robots += 1;
                new_state.materials -= blueprint.geode_robot;
                res.push(new_state);
            }
        }

        res
    }
}

fn turns_to_get(curr: u32, growth: u32, wanted: u32) -> u32 {
    if curr >= wanted {
        0
    } else {
        let (div, rem) = (wanted - curr).div_rem(&growth);
        if rem == 0 {
            div
        } else {
            div + 1
        }
    }
}

impl Blueprint {
    fn time_to_make_ore_robot(&self, state: State) -> u32 {
        turns_to_get(
            state.materials.ore,
            state.robots.ore_robots,
            self.ore_robot.ore,
        ) + 1
    }

    fn time_to_make_clay_robot(&self, state: State) -> u32 {
        turns_to_get(
            state.materials.ore,
            state.robots.ore_robots,
            self.clay_robot.ore,
        ) + 1
    }

    fn time_to_make_obsidian_robot(&self, state: State) -> u32 {
        max(
            turns_to_get(
                state.materials.ore,
                state.robots.ore_robots,
                self.obsidian_robot.ore,
            ),
            turns_to_get(
                state.materials.clay,
                state.robots.clay_robots,
                self.obsidian_robot.clay,
            ),
        ) + 1
    }

    fn time_to_make_geode_robot(&self, state: State) -> u32 {
        max(
            turns_to_get(
                state.materials.ore,
                state.robots.ore_robots,
                self.geode_robot.ore,
            ),
            turns_to_get(
                state.materials.obsidian,
                state.robots.obsidian_robots,
                self.geode_robot.obsidian,
            ),
        ) + 1
    }
}

struct Turns(u32);

impl Sub<Turns> for State {
    type Output = State;

    fn sub(self, Turns(turns): Turns) -> Self::Output {
        let mut new_state = self;
        #[allow(clippy::suspicious_arithmetic_impl)]
        for _ in 0..turns {
            new_state.materials += new_state.robots;
        }
        new_state.turns_remaining -= turns;
        new_state
    }
}

impl AddAssign<Robots> for Materials {
    fn add_assign(&mut self, rhs: Robots) {
        self.ore += rhs.ore_robots;
        self.clay += rhs.clay_robots;
        self.obsidian += rhs.obsidian_robots;
        self.geode += rhs.geode_robots;
    }
}

impl SubAssign<OreRobotCost> for Materials {
    fn sub_assign(&mut self, rhs: OreRobotCost) {
        self.ore -= rhs.ore;
    }
}

impl SubAssign<ClayRobotCost> for Materials {
    fn sub_assign(&mut self, rhs: ClayRobotCost) {
        self.ore -= rhs.ore;
    }
}

impl SubAssign<ObsidianRobotCost> for Materials {
    fn sub_assign(&mut self, rhs: ObsidianRobotCost) {
        self.ore -= rhs.ore;
        self.clay -= rhs.clay;
    }
}

impl SubAssign<GeodeRobotCost> for Materials {
    fn sub_assign(&mut self, rhs: GeodeRobotCost) {
        self.ore -= rhs.ore;
        self.obsidian -= rhs.obsidian;
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Hash)]
struct PurchaseTimes {
    ore: Vec<u32>,
    clay: Vec<u32>,
    obsidian: Vec<u32>,
    geode: Vec<u32>,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct State {
    turns_remaining: u32,
    materials: Materials,
    robots: Robots,
}

impl Default for State {
    fn default() -> Self {
        State {
            turns_remaining: 24,
            materials: Materials {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            robots: Robots {
                ore_robots: 1,
                clay_robots: 0,
                obsidian_robots: 0,
                geode_robots: 0,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Materials {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Robots {
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

#[derive(Debug, Default, Copy, Clone)]
struct Blueprint {
    number: u32,
    ore_robot: OreRobotCost,
    clay_robot: ClayRobotCost,
    obsidian_robot: ObsidianRobotCost,
    geode_robot: GeodeRobotCost,
}

#[derive(Debug, Default, Copy, Clone)]
struct OreRobotCost {
    ore: u32,
}

#[derive(Debug, Default, Copy, Clone)]
struct ClayRobotCost {
    ore: u32,
}

#[derive(Debug, Default, Copy, Clone)]
struct ObsidianRobotCost {
    ore: u32,
    clay: u32,
}

#[derive(Debug, Default, Copy, Clone)]
struct GeodeRobotCost {
    ore: u32,
    obsidian: u32,
}

impl Blueprint {
    fn parse_line(line: &str) -> IResult<&str, Blueprint> {
        map(
            tuple((
                tag("Blueprint "),
                nom_u32,
                tag(": Each ore robot costs "),
                nom_u32,
                tag(" ore. Each clay robot costs "),
                nom_u32,
                tag(" ore. Each obsidian robot costs "),
                nom_u32,
                tag(" ore and "),
                nom_u32,
                tag(" clay. Each geode robot costs "),
                nom_u32,
                tag(" ore and "),
                nom_u32,
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
                ore_robot: OreRobotCost { ore: ore_ore },
                clay_robot: ClayRobotCost { ore: clay_ore },
                obsidian_robot: ObsidianRobotCost {
                    ore: obsidian_ore,
                    clay: obsidian_clay,
                },
                geode_robot: GeodeRobotCost {
                    ore: geode_ore,
                    obsidian: geode_obsidian,
                },
            },
        )(line)
    }
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
                part1: Some(33.into()),
                part2: Some(3_472.into()),
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/19.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(2_301.into()),
                part2: Some(10_336.into()),
            }
        );
    }
}
