use std::hint::unreachable_unchecked;
use crate::{Answers, DayResult};

use bstr::{BStr, ByteSlice};

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    let mut horizontal: i32 = 0;
    let mut part1depth_and_part2aim: i32 = 0;
    let mut part2depth: i32 = 0;

    let input = BStr::new(input);

    for line in input.lines() {
        match line[0] {
            b'f' => {
                let delta: i32 = line[8].into();
                // let delta: i32 = byte_slice_to_int(&line[8..]);
                horizontal += delta;
                part2depth += part1depth_and_part2aim * delta;
            }
            b'd' => {
                part1depth_and_part2aim += Into::<i32>::into(line[5]);
                // part1depth_and_part2aim += byte_slice_to_int::<i32>(&line[5..]);
            }
            b'u' => {
                part1depth_and_part2aim -= Into::<i32>::into(line[3]);
                // part1depth_and_part2aim -= byte_slice_to_int::<i32>(&line[3..]);
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }

    Ok(DayResult {
        part1: Some(Answers::I32(horizontal * part1depth_and_part2aim)),
        part2: Some(Answers::I32(horizontal * part2depth)),
    })
}

