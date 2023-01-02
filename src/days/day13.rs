use crate::{DayResult, IntoDayResult};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

pub fn run(mut input: &'static str, _: bool) -> anyhow::Result<DayResult> {
    let div_1 = Packet(vec![Item::Packet(Packet(vec![Item::Value(2)]))]);
    let div_2 = Packet(vec![Item::Packet(Packet(vec![Item::Value(6)]))]);

    let mut pair = 1;

    let mut part1 = 0;
    let mut d1 = 1;
    let mut d2 = 2;
    loop {
        let (_input, (x, y)) = parse_packets(input)?;

        if x < y {
            part1 += pair;
        }
        pair += 1;

        if x < div_1 {
            d1 += 1;
        }
        if y < div_1 {
            d1 += 1;
        }

        if x < div_2 {
            d2 += 1;
        }
        if y < div_2 {
            d2 += 1;
        }

        if _input.is_empty() {
            break;
        }
        input = &_input[1..];
    }

    (part1, d1 * d2).into_result()
}

#[derive(Eq, PartialEq, Clone)]
struct Packet(Vec<Item>);

impl Debug for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        slice_cmp(&self.0, &other.0)
    }
}

fn slice_cmp(a: &[Item], b: &[Item]) -> Ordering {
    for (a, b) in a.iter().zip(b.iter()) {
        return match Ord::cmp(a, b) {
            Ordering::Equal => continue,
            lt_or_gt => lt_or_gt,
        };
    }

    Ord::cmp(&a.len(), &b.len())
}

fn parse_packets(input: &str) -> IResult<&str, (Packet, Packet)> {
    map(
        tuple((parse_packet, tag("\n"), parse_packet, tag("\n"))),
        |(a, _, b, _)| (a, b),
    )(input)
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    map(
        delimited(tag("["), separated_list0(tag(","), parse_item), tag("]")),
        Packet,
    )(input)
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    alt((
        map(nom::character::complete::u64, Item::Value),
        map(parse_packet, Item::Packet),
    ))(input)
}

#[derive(Eq, PartialEq, Clone)]
enum Item {
    Value(u64),
    Packet(Packet),
}

impl PartialOrd<Self> for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::Value(v1), Item::Value(v2)) => v1.cmp(v2),
            (Item::Value(v), Item::Packet(p)) => slice_cmp(&[Item::Value(*v)], &p.0),
            (Item::Packet(_), Item::Value(_)) => other.cmp(self).reverse(),
            (Item::Packet(p1), Item::Packet(p2)) => p1.cmp(p2),
        }
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Value(v) => write!(f, "{}", v),
            Item::Packet(p) => write!(f, "{:?}", p),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::DayResult;
    use std::assert_eq;

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/13.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(13.into()),
                part2: Some(140.into()),
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/13.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(4_894.into()),
                part2: Some(24_180.into()),
            }
        );
    }
}
