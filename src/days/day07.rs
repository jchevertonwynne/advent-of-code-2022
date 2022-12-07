use crate::{DayResult, IntoDayResult};
use nom::Slice;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    let fs = load_filesystem(input);
    let (sum, part1) = find_dir_sizes(&fs);
    let part2 = find_dir_to_delete(&fs, sum);
    (part1, part2).into_result()
}

fn find_dir_sizes(dir: &RefCell<Entry>) -> (usize, usize) {
    let mut dir = dir.borrow_mut();

    let mut total = dir.size;
    let mut small_sum = 0;
    for child in &dir.contents {
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

fn find_dir_to_delete(dir: &RefCell<Entry>, sum: usize) -> usize {
    find_smallest_above_size(dir, sum, usize::MAX)
}

fn find_smallest_above_size(dir: &RefCell<Entry>, tot_size: usize, best: usize) -> usize {
    let entry = dir.borrow();

    let size = entry.size;
    let mut best = best;
    let new_size = tot_size - size;

    if new_size > 40_000_000 {
        return best;
    }

    if size < best {
        best = size;
    }

    for child in &entry.contents {
        best = find_smallest_above_size(child, tot_size, best);
    }

    best
}

fn load_filesystem(input: &str) -> Rc<RefCell<Entry>> {
    let lines = input.lines().collect::<Vec<_>>();

    let res = Rc::new(RefCell::new(Entry {
        parent: Default::default(),
        size: 0,
        contents: Vec::new(),
    }));
    let mut dir = Rc::clone(&res);

    let mut i = 1;
    while i < lines.len() {
        let line = lines[i];

        if line.starts_with("$ cd") {
            i += 1;
            if line.slice(5..) == ".." {
                let new_dir = dir.borrow().parent.upgrade().unwrap();
                dir = new_dir;
                continue;
            }

            let new_dir = Rc::new(RefCell::new(Entry {
                parent: Rc::downgrade(&dir),
                size: 0,
                contents: Vec::new(),
            }));

            dir.borrow_mut().contents.push(Rc::clone(&new_dir));

            dir = new_dir;
        } else if line.starts_with("$ ls") {
            i += 1;
            while i < lines.len() {
                let line = lines[i];
                if line.starts_with('$') {
                    break;
                }

                i += 1;

                let Some((a, _b)) = line.split_once(' ') else {
                    break;
                };

                if a == "dir" {
                    continue;
                }

                let Ok(file_size) = a.parse::<usize>() else {
                    break;
                };

                dir.borrow_mut().size += file_size;
            }
        }
    }

    res
}

#[derive(Debug)]
struct Entry<'a> {
    parent: Weak<RefCell<Entry<'a>>>,
    size: usize,
    contents: Vec<Rc<RefCell<Entry<'a>>>>,
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::DayResult;

    #[test]
    fn test_example_answers() {
        let result = run(include_str!("../../input/test/07.txt"));
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
        let result = run(include_str!("../../input/real/07.txt"));
        assert_eq!(
            result.unwrap(),
            DayResult {
                part1: Some(1297159.into()),
                part2: Some(3866390.into()),
            }
        );
    }
}
