use crate::days::Answers;
use crate::days::DayResult;

const P1_MULT: [u64; 7] = create_table(80);
const P2_MULT: [u64; 7] = create_table(256);

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    let buckets = create_buckets(input);

    Ok(DayResult {
        part1: Some(Answers::Int(
            P1_MULT.iter().zip(buckets).map(|(a, b)| a * b).sum(),
        )),
        part2: Some(Answers::Int(
            P2_MULT.iter().zip(buckets).map(|(a, b)| a * b).sum(),
        )),
    })
}

fn create_buckets(input: &str) -> [u64; 9] {
    let mut buckets: [u64; 9] = Default::default();

    for &byte in input.as_bytes().iter().step_by(2) {
        buckets[(byte as usize) - ('0' as usize)] += 1;
    }

    buckets
}

const fn create_table(turns: usize) -> [u64; 7] {
    let mut workspace: [[u64; 9]; 9] = [[0; 9]; 9];

    let mut i: usize = 0;
    while i < workspace[0].len() {
        workspace[i][i] = 1;
        i += 1;
    }

    let mut repeats: usize = 0;
    while repeats < turns {
        workspace = [
            workspace[1],
            workspace[2],
            workspace[3],
            workspace[4],
            workspace[5],
            workspace[6],
            workspace[7],
            workspace[8],
            workspace[0],
        ];
        i = 0;
        while i < workspace[6].len() {
            workspace[6][i] += workspace[8][i];
            i += 1;
        }
        repeats += 1;
    }

    let mut table = [0; 7];
    i = 0;
    while i < table.len() {
        let mut j: usize = 0;
        while j < workspace[0].len() {
            table[i] += workspace[j][i];
            j += 1;
        }
        i += 1;
    }

    table
}
