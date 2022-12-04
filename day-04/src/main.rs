use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_DATA: &'static str = "./data/input";

#[derive(Clone, Copy, Debug)]
struct Assignment {
    start: u32,
    end: u32,
}

impl From<&str> for Assignment {
    fn from(value: &str) -> Self {
        let split_value: Vec<u32> = value
            .split("-")
            .map(|val| val.parse::<u32>().expect("Not a number"))
            .collect();
        Assignment {
            start: split_value[0],
            end: split_value[1],
        }
    }
}

trait Overlap<T> {
    fn overlaps_fully(&self, other: T) -> bool;
    fn overlaps(&self, other: T) -> bool;
}

impl Overlap<Assignment> for Assignment {
    fn overlaps_fully(&self, other: Assignment) -> bool {
        (self.start <= other.start && self.end >= other.end)
            || (other.start <= self.start && other.end >= self.end)
    }
    fn overlaps(&self, other: Assignment) -> bool {
        !(self.end < other.start || self.start > other.end)
    }
}

fn check_full_overlap(assignment_string: String) -> u32 {
    let assignment_pair: Vec<Assignment> = assignment_string
        .split(",")
        .map(|val| Assignment::from(val))
        .collect();
    assignment_pair[0].overlaps_fully(assignment_pair[1]) as u32
}

fn check_overlap(assignment_string: String) -> u32 {
    let assignment_pair: Vec<Assignment> = assignment_string
        .split(",")
        .map(|val| Assignment::from(val))
        .collect();
    assignment_pair[0].overlaps(assignment_pair[1]) as u32
}

fn solve_1() {
    let file = File::open(INPUT_DATA).expect("Couldn't open file");
    let buf = BufReader::new(file);
    let overlaps: u32 = buf
        .lines()
        .map(|val| check_full_overlap(val.expect("Couldn't read")))
        .sum();
    println!("{:?}", overlaps)
}

fn solve_2() {
    let file = File::open(INPUT_DATA).expect("Couldn't open file");
    let buf = BufReader::new(file);
    let overlaps: u32 = buf
        .lines()
        .map(|val| check_overlap(val.expect("Couldn't read")))
        .sum();
    println!("{:?}", overlaps)
}

fn main() {
    solve_1();
    solve_2();
}
