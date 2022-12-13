use crate::{DayResult, IntoDayResult};
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::iter::Peekable;

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    let inputs: Vec<_> = input
        .split("\n\n")
        .map(|lines| {
            let (a, b) = lines.split_once('\n').unwrap();
            (
                parse_packet(&mut a.as_bytes().iter().copied().peekable()),
                parse_packet(&mut b.as_bytes().iter().copied().peekable()),
            )
        })
        .collect();

    let part1: usize = inputs
        .iter()
        .enumerate()
        .filter(|(_, (a, b))| a.cmp(b) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum();

    let div_1 = Packet(vec![Item::Packet(Packet(vec![Item::Value(2)]))]);
    let div_2 = Packet(vec![Item::Packet(Packet(vec![Item::Value(6)]))]);

    let (d1, d2) = inputs
        .iter()
        .flat_map(|(a, b)| [a, b].into_iter())
        .fold((1, 2), |(mut d1, mut d2), packet| {
            if packet.cmp(&div_1) == Ordering::Less {
                d1 += 1
            };
            if packet.cmp(&div_2) == Ordering::Less {
                d2 += 1
            }
            (d1, d2)
        });

    let part2 = d1 * d2;

    (part1, part2).into_result()
}

fn parse_packet<I: Iterator<Item = u8>>(input: &mut Peekable<I>) -> Packet {
    let mut items = Vec::new();

    while let Some(val) = input.next() {
        match val {
            b',' | b'[' => {
                if let Some(item) = parse_item(input) {
                    items.push(item);
                }
            }
            b']' => return Packet(items),
            _ => {
                println!("bad char: {}", val as char);
                unreachable!()
            }
        }
    }

    Packet(items)
}

fn parse_item<I: Iterator<Item = u8>>(input: &mut Peekable<I>) -> Option<Item> {
    let first = *input.peek().unwrap();

    if (b'0'..=b'9').contains(&first) {
        input.next();
        let mut b = first - b'0';
        if b == 1 {
            let next = *input.peek().unwrap();
            if next == b'0' {
                b = 10;
                input.next();
            }
        }
        Some(Item::Value(b as usize))
    } else if first == b']' {
        None
    } else {
        Some(Item::Packet(parse_packet(input)))
    }
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
        match a.cmp(b) {
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => {}
            Ordering::Greater => return Ordering::Greater,
        }
    }

    a.len().cmp(&b.len())
}

#[derive(Eq, PartialEq, Clone)]
enum Item {
    Value(usize),
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
            (Item::Value(v), Item::Packet(p)) => {
                slice_cmp(&[Item::Value(*v)], &p.0)
            }
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

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/13.txt"));
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
        let result = run(include_str!("../../input/real/13.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(4894.into()),
                part2: Some(24180.into()),
            }
        );
    }
}
