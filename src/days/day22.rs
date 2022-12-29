use crate::days::day22::GroundType::{Floor, Wall};
use crate::{DayResult, IntoDayResult};
use anyhow::Context;
use fxhash::FxBuildHasher;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::hash::BuildHasher;
use std::ops::{Add, Mul, Neg};
use Direction::{Left, Right};

pub fn run(input: &'static str, _: bool) -> anyhow::Result<DayResult> {
    let (world_map, instructions) = input
        .split_once("\n\n")
        .context("failed to find double newline")?;

    let mut world = HashMap::with_hasher(FxBuildHasher::default());

    for (y, line) in world_map.lines().enumerate() {
        for (x, b) in line.as_bytes().iter().copied().enumerate() {
            let x = x as i64;
            let y = y as i64;
            let coord = Point { x, y };
            match b {
                b'.' => {
                    world.insert(coord, Floor);
                }
                b'#' => {
                    world.insert(coord, Wall);
                }
                b' ' => {}
                _ => unreachable!(),
            }
        }
    }

    let side_len = world_map
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .filter(|&&b| matches!(b, b'.' | b'#'))
                .count()
        })
        .min()
        .context("expected at least one line")? as i64;

    let position = (0..200)
        .map(|x| Point { x, y: 0 })
        .find(|p| world.contains_key(p))
        .context("failed to find start coord")?;

    let velocity = Point { x: 1, y: 0 };

    let instructions = instructions.trim();

    let part1 = solver(
        instructions,
        position,
        velocity,
        &world,
        side_len,
        try_move_p1,
    )?;
    let part2 = solver(
        instructions,
        position,
        velocity,
        &world,
        side_len,
        try_move_p2,
    )?;

    (part1, part2).into_result()
}

fn solver<H: BuildHasher>(
    mut instructions: &'static str,
    mut position: Point,
    mut velocity: Point,
    world: &HashMap<Point, GroundType, H>,
    side_len: i64,
    move_fn: impl Fn(Point, &mut Point, &HashMap<Point, GroundType, H>, i64) -> Option<Point>,
) -> anyhow::Result<i64> {
    let mut to_move = true;
    while !instructions.is_empty() {
        if to_move {
            let (_instructions, dist): (&str, i64) = parse_dist(instructions)?;
            instructions = _instructions;
            for _ in 0..dist {
                let Some(new_position) = move_fn(position, &mut velocity, world, side_len)  else { break };
                position = new_position;
            }
        } else {
            let (_instructions, direction): (&str, Direction) = parse_direction(instructions)?;
            instructions = _instructions;

            let new_velocity = match direction {
                Left => velocity.left(),
                Right => velocity.right(),
            };

            velocity = new_velocity;
        }

        to_move = !to_move;
    }

    let facing = match velocity {
        Point { x: 1, y: 0 } => 0,  // right
        Point { x: 0, y: 1 } => 1,  // down
        Point { x: -1, y: 0 } => 2, // left
        Point { x: 0, y: -1 } => 3, // up
        _ => unreachable!(),
    };

    let row = position.y + 1;
    let col = position.x + 1;

    Ok(1000 * row + 4 * col + facing)
}

fn try_move_p1(
    position: Point,
    velocity: &mut Point,
    world: &HashMap<Point, GroundType, impl BuildHasher>,
    side_len: i64,
) -> Option<Point> {
    let front_pos = position + *velocity;
    match world.get(&front_pos) {
        Some(Floor) => Some(front_pos),
        Some(Wall) => None,
        None => {
            let flipped_velocity = velocity.neg();
            for i in (1..=4).rev() {
                let potential = front_pos + flipped_velocity * side_len * i;
                if let Some(tile) = world.get(&potential) {
                    return match tile {
                        Wall => None,
                        Floor => Some(potential),
                    };
                }
            }
            None
        }
    }
}

fn try_move_p2(
    position: Point,
    velocity: &mut Point,
    world: &HashMap<Point, GroundType, impl BuildHasher>,
    side_len: i64,
) -> Option<Point> {
    println!("{position:?}");
    let front_pos = position + *velocity;
    match world.get(&front_pos) {
        Some(Floor) => Some(front_pos),
        Some(Wall) => None,
        None => {
            let state = CubeState::default();
            let (wanted, dir) = match velocity {
                Point { x: 1, y: 0 } => (state.right, MapMove::Right), // right
                Point { x: 0, y: 1 } => (state.bottom, MapMove::Down), // down
                Point { x: -1, y: 0 } => (state.left, MapMove::Left),  // left
                Point { x: 0, y: -1 } => (state.top, MapMove::Up),     // up
                _ => unreachable!(),
            };

            let mut visited = HashSet::with_capacity_and_hasher(6, FxBuildHasher::default());
            visited.insert(position);
            if let Some((p, turns)) = cube_search(
                position,
                position,
                state,
                dir,
                wanted,
                side_len,
                world,
                &mut visited,
            ) {
                for _ in 0..turns {
                    *velocity = velocity.right();
                }
                Some(p)
            } else {
                None
            }
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct CubeState {
    front: u8,
    back: u8,
    left: u8,
    right: u8,
    top: u8,
    bottom: u8,
}

impl Default for CubeState {
    fn default() -> Self {
        CubeState {
            front: 1,
            back: 2,
            left: 3,
            right: 4,
            top: 5,
            bottom: 6,
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn cube_search(
    origin: Point,
    point: Point,
    state: CubeState,
    original_dir: MapMove,
    goal: u8,
    side_len: i64,
    world: &HashMap<Point, GroundType, impl BuildHasher>,
    visited: &mut HashSet<Point, impl BuildHasher>,
) -> Option<(Point, i64)> {
    // i64 = turns to right
    if state.front == goal {
        let dir_to_original = match (state.left, state.right, state.top, state.bottom) {
            (1, _, _, _) => MapMove::Left,
            (_, 1, _, _) => MapMove::Right,
            (_, _, 1, _) => MapMove::Up,
            (_, _, _, 1) => MapMove::Down,
            _ => unreachable!(),
        };

        let origin_distances = Point {
            x: origin.x % side_len,
            y: origin.y % side_len,
        };

        let top_of_square = Point {
            x: (point.x / side_len) * side_len,
            y: (point.y / side_len) * side_len,
        };

        let (point, turns) = match (original_dir, dir_to_original) {
            (MapMove::Left, MapMove::Left) => (
                top_of_square
                    + Point {
                        x: 0,
                        y: (side_len - 1) - origin_distances.y,
                    },
                2,
            ),
            (MapMove::Left, MapMove::Right) => (
                top_of_square
                    + Point {
                        x: side_len - 1,
                        y: origin_distances.y,
                    },
                0,
            ),
            (MapMove::Left, MapMove::Up) => (
                top_of_square
                    + Point {
                        x: origin_distances.y,
                        y: 0,
                    },
                3,
            ),
            (MapMove::Left, MapMove::Down) => (
                top_of_square
                    + Point {
                        x: (side_len - 1) - origin_distances.y,
                        y: side_len - 1,
                    },
                1,
            ),
            (MapMove::Right, MapMove::Left) => (
                top_of_square
                    + Point {
                        x: 0,
                        y: origin_distances.y,
                    },
                0,
            ),
            (MapMove::Right, MapMove::Right) => (
                top_of_square
                    + Point {
                        x: side_len - 1,
                        y: (side_len - 1) - origin_distances.y,
                    },
                2,
            ),
            (MapMove::Right, MapMove::Up) => (
                top_of_square
                    + Point {
                        x: (side_len - 1) - origin_distances.y,
                        y: 0,
                    },
                1,
            ),
            (MapMove::Right, MapMove::Down) => (
                top_of_square
                    + Point {
                        x: origin_distances.y,
                        y: side_len - 1,
                    },
                3,
            ),
            (MapMove::Up, MapMove::Left) => (
                top_of_square
                    + Point {
                        x: 0,
                        y: origin_distances.x,
                    },
                1,
            ),
            (MapMove::Up, MapMove::Right) => (
                top_of_square
                    + Point {
                        x: side_len - 1,
                        y: (side_len - 1) - origin_distances.x,
                    },
                3,
            ),
            (MapMove::Up, MapMove::Up) => (
                top_of_square
                    + Point {
                        x: (side_len - 1) - origin_distances.x,
                        y: 0,
                    },
                2,
            ),
            (MapMove::Up, MapMove::Down) => (
                top_of_square
                    + Point {
                        x: origin_distances.x,
                        y: side_len - 1,
                    },
                0,
            ),
            (MapMove::Down, MapMove::Left) => (
                top_of_square
                    + Point {
                        x: 0,
                        y: (side_len - 1) - origin_distances.x,
                    },
                3,
            ),
            (MapMove::Down, MapMove::Right) => (
                top_of_square
                    + Point {
                        x: side_len - 1,
                        y: origin_distances.x,
                    },
                1,
            ),
            (MapMove::Down, MapMove::Up) => (
                top_of_square
                    + Point {
                        x: origin_distances.x,
                        y: 0,
                    },
                0,
            ),
            (MapMove::Down, MapMove::Down) => (
                top_of_square
                    + Point {
                        x: (side_len - 1) - origin_distances.x,
                        y: side_len - 1,
                    },
                2,
            ),
        };
        return match world.get(&point) {
            None => unreachable!(),
            Some(Wall) => None,
            Some(Floor) => Some((point, turns)),
        };
    }

    let up = point + Point { x: 0, y: -side_len };
    if world.contains_key(&up) && visited.insert(up) {
        let new_state = CubeState {
            top: state.back,
            back: state.bottom,
            bottom: state.front,
            front: state.top,
            ..state
        };
        if let solution @ Some(_) = cube_search(
            origin,
            up,
            new_state,
            original_dir,
            goal,
            side_len,
            world,
            visited,
        ) {
            return solution;
        }
    }

    let down = point + Point { x: 0, y: side_len };
    if world.contains_key(&down) && visited.insert(down) {
        let new_state = CubeState {
            top: state.front,
            back: state.top,
            bottom: state.back,
            front: state.bottom,
            ..state
        };
        if let solution @ Some(_) = cube_search(
            origin,
            down,
            new_state,
            original_dir,
            goal,
            side_len,
            world,
            visited,
        ) {
            return solution;
        }
    }

    let left = point + Point { x: -side_len, y: 0 };
    if world.contains_key(&left) && visited.insert(left) {
        let new_state = CubeState {
            left: state.back,
            back: state.right,
            right: state.front,
            front: state.left,
            ..state
        };
        if let solution @ Some(_) = cube_search(
            origin,
            left,
            new_state,
            original_dir,
            goal,
            side_len,
            world,
            visited,
        ) {
            return solution;
        }
    }

    let right = point + Point { x: side_len, y: 0 };
    if world.contains_key(&right) && visited.insert(right) {
        let new_state = CubeState {
            left: state.front,
            back: state.left,
            right: state.back,
            front: state.right,
            ..state
        };
        if let solution @ Some(_) = cube_search(
            origin,
            right,
            new_state,
            original_dir,
            goal,
            side_len,
            world,
            visited,
        ) {
            return solution;
        }
    }

    None
}

#[derive(Debug)]
enum GroundType {
    Wall,
    Floor,
}

fn parse_dist(line: &str) -> IResult<&str, i64> {
    nom::character::complete::i64(line)
}

fn parse_direction(line: &str) -> IResult<&str, Direction> {
    alt((map(tag("L"), |_| Left), map(tag("R"), |_| Right)))(line)
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
enum MapMove {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Point {
    x: i64,
    y: i64,
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}, {}}}", self.x, self.y)
    }
}

impl Point {
    fn left(&self) -> Point {
        let &Point { x, y } = self;
        Point { x: y, y: -x }
    }

    fn right(&self) -> Point {
        let &Point { x, y } = self;
        Point { x: -y, y: x }
    }
}

impl Mul<i64> for Point {
    type Output = Point;

    fn mul(self, rhs: i64) -> Self::Output {
        let Point { x, y } = self;
        Point {
            x: x * rhs,
            y: y * rhs,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        let Point { x, y } = self;
        Point { x: -x, y: -y }
    }
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::DayResult;

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/22.txt"), false);
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
        let result = run(include_str!("../../input/real/22.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: None,
                part2: None,
            }
        );
    }
}
