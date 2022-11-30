use crate::days::day03::Side::{Left, Right};
use crate::{DayResult, IntoDayResult};
use anyhow::Context;
use nonmax::NonMaxU8;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use thiserror::Error;

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    let snails = input
        .lines()
        .map(TryFrom::try_from)
        .collect::<Result<Vec<SnailNumber>, SnailNumberParseError>>()?;

    let part1 = snails[1..]
        .iter()
        .fold(snails[0], |a, &b| a + b)
        .magnitude();

    let part2 = snails
        .iter()
        .enumerate()
        .flat_map(|(i, &snail1)| snails[i + 1..].iter().map(move |&snail2| (snail1, snail2)))
        .map(|(a, b)| a + b)
        .map(|s| s.magnitude())
        .max()
        .context("expected a snail")?;

    (part1, part2).into_result()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct SnailNumber([Option<NonMaxU8>; 64]);

impl<'a> TryFrom<&'a str> for SnailNumber {
    type Error = SnailNumberParseError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut result = [None; 64];

        let mut snail_index: usize = 0;
        for &byte in value.as_bytes() {
            match byte {
                b'[' => snail_index = snail_index * 2 + 1,
                b']' => snail_index = (snail_index - 1) / 2,
                b',' => snail_index += 1,
                b'0'..=b'9' => {
                    result[snail_index - 1] = Some(unsafe { NonMaxU8::new_unchecked(byte - b'0') })
                }
                _ => return Err(SnailNumberParseError::InvalidCharacter),
            }
        }

        Ok(SnailNumber(result))
    }
}

#[derive(Debug, Error)]
enum SnailNumberParseError {
    #[error("Input contained an invalid character")]
    InvalidCharacter,
}

impl Display for SnailNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self._print(0, f)
    }
}

impl Add for SnailNumber {
    type Output = SnailNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = SnailNumber([None; 64]);

        result.fill(1, 0, 0, &self);
        result.fill(2, 0, 0, &rhs);
        result.normalise();

        result
    }
}

impl SnailNumber {
    fn _print(&self, index: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
        if index == 0 || self.0[index - 1].is_none() {
            write!(f, "[")?;
            self._print(index * 2 + 1, f)?;
            write!(f, ",")?;
            self._print(index * 2 + 2, f)?;
            write!(f, "]")
        } else {
            let value = self.0[index - 1].unwrap();
            write!(f, "{}", value)
        }
    }

    fn fill(&mut self, base: usize, ind: usize, depth: usize, from: &SnailNumber) {
        if depth > 4 {
            return;
        }

        if ind != 0 {
            self.0[base - 1] = from.0[ind - 1];
        }

        self.fill(base * 2 + 1, ind * 2 + 1, depth + 1, from);
        self.fill(base * 2 + 2, ind * 2 + 2, depth + 1, from);
    }

    fn normalise(&mut self) {
        loop {
            while self.explode() {}
            if !self.split() {
                return;
            }
        }
    }

    fn add_value(&mut self, side: Side, index: usize, value: u8) {
        if let Some(val) = self.0[index - 1].as_mut() {
            let raw_val = val.get();
            *val = unsafe { NonMaxU8::new_unchecked(raw_val + value) }
        } else {
            match side {
                Left => self.add_value(side, index * 2 + 2, value),
                Right => self.add_value(side, index * 2 + 1, value),
            }
        }
    }

    fn magnitude(&self) -> u64 {
        self._magnitude(0)
    }

    fn _magnitude(&self, i: usize) -> u64 {
        if i != 0 {
            if let Some(val) = self.0[i - 1] {
                return val.get() as u64;
            }
        }

        3 * self._magnitude(i * 2 + 1) + 2 * self._magnitude(i * 2 + 2)
    }

    fn explode(&mut self) -> bool {
        self._explode(0, 0).explosion_done
    }

    fn _explode(&mut self, i: usize, depth: usize) -> ExplodeResult {
        if depth == 4 && self.0[i - 1].is_none() {
            let mut left = self.0[i * 2 + 1 - 1];
            let mut right = self.0[i * 2 + 2 - 1];

            if i % 2 == 0 {
                let left_val = left.expect("a valid snail number always has a value here for this condition to trigger").get();
                self.add_value(Right, i - 1, left_val);
                left = None;
            } else {
                let right_val = right.expect("a valid snail number always has a value here for this condition to trigger").get();
                self.add_value(Right, i + 1, right_val);
                right = None;
            }

            return ExplodeResult {
                left,
                right,
                explosion_done: true,
                source: true,
            };
        }

        let mut result = ExplodeResult {
            left: None,
            right: None,
            explosion_done: false,
            source: false,
        };

        if i != 0 && self.0[i - 1].is_some() {
            return result;
        }

        let left_result = self._explode(i * 2 + 1, depth + 1);
        result.explosion_done |= left_result.explosion_done;
        if left_result.source {
            self.0[i * 2 + 1 - 1] = Some(NonMaxU8::default());
        }

        self._explode_result_handler(i, &left_result, &mut result);

        if left_result.explosion_done {
            return result;
        }

        let right_result = self._explode(i * 2 + 2, depth + 1);
        result.explosion_done |= right_result.explosion_done;
        if right_result.source {
            self.0[i * 2 + 2 - 1] = Some(NonMaxU8::default());
        }

        self._explode_result_handler(i, &right_result, &mut result);

        result
    }

    fn _explode_result_handler(
        &mut self,
        i: usize,
        side_result: &ExplodeResult,
        result: &mut ExplodeResult,
    ) {
        if let Some(left) = side_result.left {
            if i % 2 == 0 && i != 0 {
                self.add_value(Left, i - 1, left.get());
            } else {
                result.left = Some(left);
            }
        }

        if let Some(right) = side_result.right {
            if i % 2 == 1 {
                self.add_value(Right, i + 1, right.get());
            } else {
                result.right = Some(right);
            }
        }
    }

    fn split(&mut self) -> bool {
        self._split(0)
    }

    fn _split(&mut self, i: usize) -> bool {
        if i != 0 {
            if let Some(val) = self.0[i - 1] {
                let val = val.get();
                return if val >= 10 {
                    let left = val / 2;
                    let right = val - left;
                    self.0[i - 1] = None;
                    self.0[i * 2 + 1 - 1] = Some(unsafe { NonMaxU8::new_unchecked(left) });
                    self.0[i * 2 + 2 - 1] = Some(unsafe { NonMaxU8::new_unchecked(right) });
                    true
                } else {
                    false
                };
            }
        }

        self._split(i * 2 + 1) || self._split(i * 2 + 2)
    }
}

enum Side {
    Left,
    Right,
}

struct ExplodeResult {
    left: Option<NonMaxU8>,
    right: Option<NonMaxU8>,
    explosion_done: bool,
    source: bool,
}

#[cfg(test)]
mod tests {
    use crate::{Answers, DayResult};

    use super::run;

    #[test]
    fn expected_answers() {
        let result = run(include_str!("../../input/real/03.txt"));
        assert!(matches!(
            result,
            Ok(DayResult {
                part1: Some(Answers::U64(3216)),
                part2: Some(Answers::U64(4643))
            })
        ));
    }
}
