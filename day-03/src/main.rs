use std::{
    collections::{hash_map::RandomState, hash_set::Intersection, HashSet},
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
    path::Path,
};

const INPUT_DATA: &'static str = "./data/input";
const ELF_GROUP_SIZE: usize = 3;

pub trait Priority {
    fn get_priority(&self) -> u32;
}

impl Priority for char {
    fn get_priority(&self) -> u32 {
        if !self.is_alphabetic() {
            panic!("Wrong char");
        }
        let code = *self as u32;

        if self.is_lowercase() {
            return code - 96;
        }
        code - 38
    }
}

fn load_data(path: &str) -> Vec<String> {
    let file_path = Path::new(path);
    let file = File::open(file_path).expect("Couldn't open file");

    let buf = BufReader::new(file);
    buf.lines().map(|l| l.unwrap()).collect()
}

fn solve_1() {
    let backpacks = load_data(INPUT_DATA);
    let mut prio_sum: u32 = 0;
    for backpack in backpacks {
        let backpack_size = backpack.len();
        let mut compartment_1: HashSet<char> = HashSet::new();
        let mut compartment_2: HashSet<char> = HashSet::new();

        for (idx, item) in backpack.chars().enumerate() {
            if idx < backpack_size / 2 {
                compartment_1.insert(item);
            } else {
                compartment_2.insert(item);
            }
        }

        let dupe: &char = compartment_1
            .intersection(&compartment_2)
            .collect::<HashSet<&char>>()
            .drain()
            .collect::<Vec<&char>>()[0];

        prio_sum += dupe.get_priority()
    }
    println!("{:?}", prio_sum);
}

fn solve_2() {
    let backpacks = load_data(INPUT_DATA);
    let mut prio_sum: u32 = 0;

    for elf_group in backpacks.chunks(ELF_GROUP_SIZE) {
        let item_sets: Vec<HashSet<char>> = elf_group
            .iter()
            .map(|backpack| HashSet::<char>::from_iter(backpack.chars()))
            .collect();
        let intersections: Vec<HashSet<&char>> = item_sets
            .windows(2)
            .map(|sets| sets[0].intersection(&sets[1]))
            .collect::<Vec<Intersection<char, RandomState>>>()
            .iter()
            .map(|intersection| intersection.to_owned().collect::<HashSet<&char>>())
            .collect();
        let badge: &&char = intersections[0]
            .intersection(&intersections[1])
            .collect::<HashSet<&&char>>()
            .drain()
            .collect::<Vec<&&char>>()[0];

        prio_sum += badge.get_priority();
    }
    println!("{:?}", prio_sum);
}

fn main() {
    solve_1();
    solve_2();
}
