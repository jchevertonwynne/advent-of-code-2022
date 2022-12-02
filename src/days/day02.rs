use crate::days::day02::Hand::{Paper, Rock, Scissors};
use crate::days::day02::MatchResult::{Draw, Loss, Win};
use crate::{DayResult, IntoDayResult};
use thiserror::Error;

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    let mut part1: u32 = 0;
    let mut part2: u32 = 0;

    let input = input.as_bytes();

    for (first, second) in (0..input.len())
        .step_by(4)
        .map(|i| (input[i], input[i + 2]))
    {
        let opp: Hand = first.try_into()?;
        let me: Hand = second.try_into()?;
        let intended: MatchResult = second.try_into()?;
        let res = Hand::is_win(me, opp);

        let should_achieve = match intended {
            Draw => opp.draws(),
            Win => opp.loses(),
            Loss => opp.beats(),
        };

        part1 += Into::<u32>::into(me) + Into::<u32>::into(res);
        part2 += Into::<u32>::into(should_achieve) + Into::<u32>::into(intended);
    }

    (part1, part2).into_result()
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn beats(self) -> Hand {
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn draws(self) -> Hand {
        self
    }

    fn loses(self) -> Hand {
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }
}

impl From<Hand> for u32 {
    fn from(hand: Hand) -> Self {
        match hand {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

impl TryFrom<u8> for Hand {
    type Error = HandParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'A' => Ok(Rock),
            b'B' => Ok(Paper),
            b'C' => Ok(Scissors),
            b'X' => Ok(Rock),
            b'Y' => Ok(Paper),
            b'Z' => Ok(Scissors),
            _ => Err(HandParseError::InvalidChar(value)),
        }
    }
}

#[derive(Debug, Error)]
enum HandParseError {
    #[error("an unknown char was found: {0}")]
    InvalidChar(u8),
}

impl Hand {
    fn is_win(you: Hand, opp: Hand) -> MatchResult {
        if you.loses() == opp {
            Loss
        } else if you.beats() == opp {
            Win
        } else {
            Draw
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum MatchResult {
    Draw,
    Win,
    Loss,
}

impl TryFrom<u8> for MatchResult {
    type Error = MatchResultParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'X' => Ok(Loss),
            b'Y' => Ok(Draw),
            b'Z' => Ok(Win),
            _ => Err(MatchResultParseError::InvalidChar(value)),
        }
    }
}

#[derive(Debug, Error)]
enum MatchResultParseError {
    #[error("an unknown char was found: {0}")]
    InvalidChar(u8),
}

impl From<MatchResult> for u32 {
    fn from(res: MatchResult) -> Self {
        match res {
            Draw => 3,
            Win => 6,
            Loss => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::{Answers, DayResult};

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/02.txt"));
        assert!(matches!(
            result,
            Ok(DayResult {
                part1: Some(Answers::U32(15)),
                part2: Some(Answers::U32(12)),
            })
        ));
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/02.txt"));
        assert!(matches!(
            result,
            Ok(DayResult {
                part1: Some(Answers::U32(9241)),
                part2: Some(Answers::U32(14610)),
            })
        ));
    }
}
