use crate::{DayResult, IntoDayResult};
use nom::Slice;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub fn run(input: &'static str) -> anyhow::Result<DayResult> {
    let fs = load_filesystem(input);
    let (sum, part1) = print_dir_sizes(Rc::clone(&fs));
    let part2 = find_dir_to_delete(fs, sum);
    (part1, part2).into_result()
}

fn print_dir_sizes(dir: Rc<RefCell<Entry>>) -> (usize, usize) {
    let mut tot = 0;
    let mut sum = 0;

    match dir.borrow().deref() {
        Entry::Directory { contents, .. } => {
            for c in contents.values() {
                let (t, s) = print_dir_sizes(Rc::clone(c));
                tot += t;
                sum += s;
            }

            if tot < 100_000 {
                sum += tot;
            }
        }
        Entry::File { size, .. } => {
            tot += size;
        }
    };

    (tot, sum)
}

fn find_dir_to_delete(dir: Rc<RefCell<Entry>>, sum: usize) -> usize {
    find_smallest_above_size(dir, sum, usize::MAX)
}

fn find_smallest_above_size(dir: Rc<RefCell<Entry>>, tot_size: usize, best: usize) -> usize {
    if let Entry::File { .. } = dir.borrow().deref() {
        return best;
    }

    let (size, _) = print_dir_sizes(Rc::clone(&dir));

    let mut best = best;
    let new_size = tot_size - size;

    if new_size < 40_000_000 && size < best {
        best = size;
    }

    if let Entry::Directory { contents, .. } = dir.borrow().deref() {
        for c in contents.values() {
            best = find_smallest_above_size(Rc::clone(c), tot_size, best);
        }
    }

    best
}

fn load_filesystem(input: &str) -> Rc<RefCell<Entry>> {
    let lines = input.lines().collect::<Vec<_>>();

    let res = Rc::new(RefCell::new(Entry::Directory {
        contents: HashMap::new(),
    }));
    let mut dir = vec![Rc::clone(&res)];

    let mut i = 1;
    while i < lines.len() {
        let line = lines[i];

        let mut was_ls = false;
        if line.starts_with("$ cd") {
            let cmd = line.slice(5..);
            match cmd {
                ".." => {
                    dir.pop();
                }
                _ => {
                    let new_dir = match dir[dir.len() - 1].borrow_mut().deref_mut() {
                        Entry::Directory { contents, .. } => {
                            let new_dir = Rc::new(RefCell::new(Entry::Directory {
                                contents: HashMap::new(),
                            }));
                            contents.entry(cmd).or_insert_with(|| Rc::clone(&new_dir));
                            new_dir
                        }
                        Entry::File { .. } => unreachable!(),
                    };
                    dir.push(new_dir);
                }
            }
        } else if line.starts_with("$ ls") {
            was_ls = true;
            i += 1;
            while i < lines.len() {
                let line = lines[i];
                if line.starts_with('$') {
                    break;
                }

                let Some((a, b)) = line.split_once(' ') else {
                    break;
                };

                if a == "dir" {
                } else {
                    match a.parse::<usize>() {
                        Ok(size) => match dir.last_mut().unwrap().borrow_mut().deref_mut() {
                            Entry::Directory { contents, .. } => {
                                let new_file = Rc::new(RefCell::new(Entry::File { size }));
                                contents.insert(b, new_file);
                            }
                            Entry::File { .. } => unreachable!(),
                        },
                        Err(_) => break,
                    }
                }

                i += 1;
            }
        }

        if !was_ls {
            i += 1;
        }
    }

    res
}

#[derive(Debug)]
enum Entry<'a> {
    Directory {
        contents: HashMap<&'a str, Rc<RefCell<Entry<'a>>>>,
    },
    File {
        size: usize,
    },
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
