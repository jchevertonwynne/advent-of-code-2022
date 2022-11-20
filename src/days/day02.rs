use crate::{Answers, DayResult};

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    let mut horizontal: i32 = 0;
    let mut part1depth_and_part2aim: i32 = 0;
    let mut part2depth: i32 = 0;

    let input = input.as_bytes();

    let mut ind: usize = 0;
    while ind < input.len() {
        let f = Into::<i32>::into(input[ind] == b'f');
        let u = Into::<i32>::into(input[ind] == b'u');
        let d = Into::<i32>::into(input[ind] == b'd');
        let num_ind = (8 * f as usize) + (4 * d as usize) + (2 * u as usize) + (1 * (d as usize | u as usize));
        let num = Into::<i32>::into(input[ind + num_ind] - b'0');

        horizontal += num * f;
        part2depth += (part1depth_and_part2aim * num) * f;
        part1depth_and_part2aim += num * (d - u);

        ind += num_ind + 2;
    }

    Ok(DayResult {
        part1: Some(Answers::I32(horizontal * part1depth_and_part2aim)),
        part2: Some(Answers::I32(horizontal * part2depth)),
    })
}
