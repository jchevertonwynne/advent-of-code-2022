use crate::{Day10Result, DayResult, IntoDayResult};
use bstr::{BStr, ByteSlice};

pub fn run(input: &'static str, _: bool) -> anyhow::Result<DayResult> {
    let mut part1 = 0;
    let mut part2 = Day10Result([0; 6]);

    let mut cycle: isize = 1;
    let mut x = 1;

    for line in BStr::new(input).lines() {
        if line == b"noop" {
            if (((cycle - 1) % 40) - x).abs() <= 1 {
                part2.0[((cycle - 1) / 40) as usize] |= 1 << ((cycle - 1) % 40);
            }
            cycle += 1;

            if cycle <= 220 && (cycle + 20) % 40 == 0 {
                part1 += cycle * x;
            }

            if cycle == 241 {
                return (part1, part2).into_result();
            }
        } else {
            if (((cycle - 1) % 40) - x).abs() <= 1 {
                part2.0[((cycle - 1) / 40) as usize] |= 1 << ((cycle - 1) % 40);
            }
            cycle += 1;

            if cycle <= 220 && (cycle + 20) % 40 == 0 {
                part1 += cycle * x;
            }

            if cycle == 241 {
                return (part1, part2).into_result();
            }

            if (((cycle - 1) % 40) - x).abs() <= 1 {
                part2.0[((cycle - 1) / 40) as usize] |= 1 << ((cycle - 1) % 40);
            }

            cycle += 1;
            x += unsafe { std::str::from_utf8_unchecked(&line[5..]) }.parse::<isize>()?;

            if cycle <= 220 && (cycle + 20) % 40 == 0 {
                part1 += cycle * x;
            }

            if cycle == 241 {
                return (part1, part2).into_result();
            }
        };
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::{Day10Result, DayResult};
    use std::assert_eq;

    fn build_p2(p2: &str) -> Day10Result {
        let mut builder = Day10Result([0; 6]);
        for (y, line) in p2.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    builder.0[y] |= 1 << x
                }
            }
        }
        builder
    }

    #[test]
    fn test_example_answers() {
        let p2 = "##  ##  ##  ##  ##  ##  ##  ##  ##  ##
###   ###   ###   ###   ###   ###   ###
####    ####    ####    ####    ####
#####     #####     #####     #####
######      ######      ######      ####
#######       #######       #######";

        let result = run(include_str!("../../input/test/10.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(13_140.into()),
                part2: Some(build_p2(p2).into()),
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/10.txt"), false);
        let p2 = "###  #### ###   ##  #### ####   ## ###
#  #    # #  # #  #    # #       # #  #
#  #   #  ###  #      #  ###     # ###
###   #   #  # # ##  #   #       # #  #
#    #    #  # #  # #    #    #  # #  #
#    #### ###   ### #### ####  ##  ###";
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(14_520.into()),
                part2: Some(build_p2(p2).into()),
            }
        );
    }
}
