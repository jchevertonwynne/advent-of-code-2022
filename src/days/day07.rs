use crate::days::byte_slice_to_int;
use crate::{DayResult, IntoDayResult};
use bstr::{BStr, ByteSlice};
use nom::Slice;

pub fn run(input: &'static str, _: bool) -> anyhow::Result<DayResult> {
    let mut fs = load_filesystem(input);
    let (sum, part1) = find_dir_sizes(&mut fs);
    let part2 = find_dir_to_delete(&fs, sum);
    (part1, part2).into_result()
}

fn find_dir_sizes(dir: &mut Entry) -> (usize, usize) {
    let mut total = dir.size;
    let mut small_sum = 0;

    for child in &mut dir.contents {
        let (sub_total, sub_small_sum) = find_dir_sizes(child);
        total += sub_total;
        small_sum += sub_small_sum;
    }

    if total < 100_000 {
        small_sum += total;
    }

    dir.size = total;

    (total, small_sum)
}

fn find_dir_to_delete(dir: &Entry, sum: usize) -> usize {
    find_smallest_above_size(dir, sum, usize::MAX)
}

fn find_smallest_above_size(dir: &Entry, tot_size: usize, best: usize) -> usize {
    let size = dir.size;
    let mut best = best;
    let new_size = tot_size - size;

    if new_size > 40_000_000 {
        return best;
    }

    if size < best {
        best = size;
    }

    for child in &dir.contents {
        best = find_smallest_above_size(child, tot_size, best);
    }

    best
}

fn load_filesystem(input: &str) -> Entry {
    let mut lines = BStr::new(input).lines();

    load_inner(&mut lines)
}

fn load_inner<'a, I: Iterator<Item = &'a [u8]>>(lines: &mut I) -> Entry {
    let mut cur_dir = Entry::default();

    let mut is_ls = false;
    while let Some(line) = lines.next() {
        if is_ls && is_ls_output(line, &mut cur_dir) {
            continue;
        }

        if line.starts_with(b"$ cd") {
            if line.slice(5..) == b".." {
                return cur_dir;
            }

            cur_dir.contents.push(load_inner(lines));
        } else if line.starts_with(b"$ ls") {
            is_ls = true;
        }
    }

    cur_dir
}

#[inline(always)]
fn is_ls_output(line: &[u8], dir: &mut Entry) -> bool {
    if line.starts_with(b"$") {
        return false;
    }

    if line.starts_with(b"dir") {
        return true;
    }

    let Some(a) = line.split(|&b| b == b' ').next() else {
        return false;
    };

    dir.size += byte_slice_to_int::<usize>(a);

    true
}

#[derive(Debug, Default)]
struct Entry {
    size: usize,
    contents: Vec<Entry>,
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::DayResult;

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/07.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(95437.into()),
                part2: Some(24933642.into()),
            }
        );
    }

    #[test]
    fn test_answers() {
        let result = run(include_str!("../../input/real/07.txt"), false);
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(1297159.into()),
                part2: Some(3866390.into()),
            }
        );
    }
}
