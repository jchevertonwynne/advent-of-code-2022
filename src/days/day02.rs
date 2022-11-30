use crate::{DayResult, IntoDayResult};

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    let mut horizontal: i32 = 0;
    let mut part1depth_and_part2aim: i32 = 0;
    let mut part2depth: i32 = 0;

    let input = input.as_bytes();

    let mut ind: usize = 0;
    while ind < input.len() {
        let f = (input[ind] == b'f') as i32;
        let u = (input[ind] == b'u') as i32;
        let d = (input[ind] == b'd') as i32;
        let num_ind =
            (8 * f as usize) | (4 * d as usize) | (2 * u as usize) | (d as usize | u as usize);
        let num = (input[ind + num_ind] - b'0') as i32;

        horizontal += num * f;
        part2depth += part1depth_and_part2aim * num * f;
        part1depth_and_part2aim += num * (d - u);

        ind += num_ind + 2;
    }

    (
        horizontal * part1depth_and_part2aim,
        horizontal & part2depth,
    )
        .into_result()
}

#[cfg(test)]
mod tests {
    use crate::{Answers, DayResult};

    use super::run;

    #[test]
    fn expected_answers() {
        let result = run(include_str!("../../input/real/02.txt"));
        assert!(matches!(
            result,
            Ok(DayResult {
                part1: Some(Answers::I32(1893605)),
                part2: Some(Answers::I32(2120734350))
            })
        ));
    }
}
