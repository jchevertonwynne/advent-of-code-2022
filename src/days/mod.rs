use std::fmt::Display;

pub mod day01;

pub enum Answers {
    String(String),
    Int(usize),
}

impl Display for Answers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Answers::String(s) => write!(f, "{}", s),
            Answers::Int(i) => write!(f, "{}", i),
        }
    }
}

pub struct DayResult {
    pub part1: Option<Answers>,
    pub part2: Option<Answers>,
}
