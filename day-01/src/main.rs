use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

static DATA_FILE: &'static str = "./data.txt";

fn load_calories(path: &str) -> Vec<u32> {
    let file_path = Path::new(path);
    let file = File::open(file_path).expect("Couldn't open file");

    let buf = BufReader::new(file);
    let mut all_elves: Vec<u32> = Vec::new();
    let mut one_elf: u32 = 0;
    let lines = buf.lines().map(|l| l.unwrap());
    for line in lines {
        match line.parse::<u32>() {
            Err(_) => {
                all_elves.push(one_elf);
                one_elf = 0;
            }
            Ok(ration_calories) => one_elf = one_elf + ration_calories,
        };
    }
    all_elves
}

fn solve_1() -> u32 {
    let calories = load_calories(DATA_FILE);
    calories.iter().max().expect("Couldn't find max").to_owned()
}

fn solve_2() -> u32 {
    let mut calories = load_calories(DATA_FILE);
    calories.sort();
    calories.iter().rev().take(3).sum()
}

fn main() {
    print!("Solution 1:  {}\n", solve_1());
    print!("Solution 2: {}\n", solve_2());
}
