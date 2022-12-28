use crate::{DayResult, IntoDayResult};

const LOOKUP_SCORE: [[usize; 3]; 3] = [
    [4, 1, 7], // rock
    [8, 5, 2], // paper
    [3, 9, 6], // scissors
];

const LOOKUP_MOVE: [[usize; 3]; 3] = [
    [3, 4, 8], // rock
    [1, 5, 9], // paper
    [2, 6, 7], // scissors
];

pub fn run(input: &'static str, _: bool) -> anyhow::Result<DayResult> {
    let input = input.as_bytes();

    (0..input.len())
        .step_by(4)
        .fold((0, 0), |(p1, p2), i| {
            let opp = (input[i] - b'A') as usize;
            let me_or_goal = (input[i + 2] - b'X') as usize;
            let p1 = p1 + LOOKUP_SCORE[me_or_goal][opp];
            let p2 = p2 + LOOKUP_MOVE[opp][me_or_goal];
            (p1, p2)
        })
        .into_result()
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::{Answers, DayResult};

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/02.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(Answers::Usize(15)),
                part2: Some(Answers::Usize(12)),
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/02.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(Answers::Usize(9_241)),
                part2: Some(Answers::Usize(14_610)),
            }
        );
    }
}
