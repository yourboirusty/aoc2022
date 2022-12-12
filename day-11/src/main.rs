use std::{
    collections::VecDeque,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};
mod math;
use math::lcm;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Addition,
    Multiplication,
}

impl From<&str> for Operator {
    fn from(value: &str) -> Self {
        match value {
            "+" => Operator::Addition,
            "*" => Operator::Multiplication,
            _ => panic!("NaP"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Operation {
    value_l: Option<u64>,
    operator: Operator,
    value_r: Option<u64>,
}

impl Operation {
    fn result(&self, val: u64) -> u64 {
        let left = self.value_l.unwrap_or(val);
        let right = self.value_r.unwrap_or(val);
        match self.operator {
            Operator::Addition => left + right,
            Operator::Multiplication => left * right,
        }
    }
}

impl From<&str> for Operation {
    fn from(val: &str) -> Self {
        let operation = val.split_once("=").unwrap().1;
        let mut operation_iter = operation.split_ascii_whitespace();
        Self {
            value_l: match operation_iter.next().unwrap() {
                "old" => None,
                val => Some(val.parse().expect("NaN")),
            },
            operator: operation_iter.next().unwrap().into(),
            value_r: match operation_iter.next().unwrap() {
                "old" => None,
                val => Some(val.parse().expect("NaN")),
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Tests {
    Divisble,
}

impl From<&str> for Tests {
    fn from(val: &str) -> Self {
        match val {
            "divisible" => Tests::Divisble,
            _ => panic!("NoT"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Test {
    test: Tests,
    value: u64,
    on_true: usize,
    on_false: usize,
}

impl Test {
    fn perform(&self, value: u64) -> usize {
        match self.test {
            Tests::Divisble => self.divisble(value),
        }
    }

    fn divisble(&self, value: u64) -> usize {
        if value % self.value == 0 {
            return self.on_true;
        }
        self.on_false
    }
}

#[derive(Clone)]
struct Monke {
    name: u64,
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    examinations: u64,
}

impl Monke {
    fn examine(&mut self) -> (usize, u64) {
        self.examinations += 1;
        let new_level = self.operation.result(self.items.pop_front().unwrap());
        let next_monke = self.test.perform(new_level);
        (next_monke, new_level)
    }

    fn therapy(&mut self, therapy_lcm: u64) {
        self.items = self.items.iter().map(|a| a % therapy_lcm).collect()
    }
}

impl Debug for Monke {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monke")
            .field("name", &self.name)
            .field("examinations", &self.examinations)
            .finish()
    }
}

fn get_lcm(barrel: &Vec<Monke>) -> u64 {
    let divisors: Vec<u64> = barrel.iter().map(|m| m.test.value).collect();
    lcm(divisors)
}

fn main() {
    let mut barrel = load_data("./data/input");
    let therapy_lcm = get_lcm(&barrel);
    for _ in 0..10_000 {
        shenanigans(&mut barrel, therapy_lcm);
    }
    barrel.sort_by(|a, b| b.examinations.cmp(&a.examinations));
    println!("{:?}", barrel);
    let top_monke_exams: Vec<u64> = barrel.iter().take(2).map(|m| m.examinations).collect();
    let mut monke_business = 1;
    for monke_exam in top_monke_exams {
        monke_business *= monke_exam;
    }
    println!("{:?}", monke_business);
}

fn shenanigans(barrel: &mut Vec<Monke>, therapy_lcm: u64) {
    for idx in 0..barrel.len() {
        let mut passed_items: Vec<(usize, u64)> = Vec::new();

        {
            let monke = barrel.get_mut(idx).unwrap();
            for _ in 0..monke.items.len() {
                let monke_pass = monke.examine();
                passed_items.push(monke_pass);
            }
        }

        for (monke_to_pass, item) in passed_items {
            barrel.get_mut(monke_to_pass).unwrap().items.push_back(item);
        }
    }
    for monke in barrel {
        monke.therapy(therapy_lcm);
    }
}

fn load_data(path: &str) -> Vec<Monke> {
    let file = File::open(path).expect("Can't open file");
    let data: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut barrel: Vec<Monke> = Vec::new();

    let monke_strings = get_strings_per_monke(data);

    for monke_string in monke_strings {
        let mut name = 0_u64;
        let mut items: VecDeque<u64> = VecDeque::new();
        let mut operation: Operation = Operation {
            value_l: None,
            value_r: None,
            operator: Operator::Addition,
        };
        let mut test: Test = Test {
            test: Tests::Divisble,
            value: 0,
            on_true: 0,
            on_false: 0,
        };
        for (command, data) in monke_string.iter().map(|v| v.split_once(':').unwrap()) {
            match command
                .split_whitespace()
                .collect::<Vec<&str>>()
                .first()
                .unwrap()
                .trim()
            {
                "Monkey" => {
                    name = command
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .last()
                        .unwrap()
                        .trim()
                        .parse()
                        .expect("NaN")
                }
                "Starting" => items = data.split(',').map(|v| v.trim().parse().unwrap()).collect(),
                "Operation" => operation = data.into(),
                "Test" => {
                    let data_vec: Vec<&str> = data.trim().split_whitespace().collect();
                    test.test = (*data_vec.first().unwrap()).into();
                    test.value = (*data_vec.last().unwrap()).parse().expect("NaN");
                }
                "If" => {
                    let data_vec: Vec<&str> = data.trim().split_whitespace().collect();
                    let value: usize = data_vec.last().unwrap().parse().expect("NaN");
                    match command
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .last()
                        .unwrap()
                        .trim()
                    {
                        "true" => test.on_true = value,
                        "false" => test.on_false = value,
                        _ => panic!("NaC"),
                    }
                }
                _ => panic!("NoC"),
            }
        }
        barrel.push(Monke {
            name: name,
            items: items.clone(),
            operation: operation.clone(),
            test: test,
            examinations: 0,
        })
    }

    barrel
}

fn get_strings_per_monke(iter: Vec<String>) -> Vec<Vec<String>> {
    let mut monke_strings: Vec<Vec<String>> = Vec::new();
    let mut monke: Vec<String> = Vec::new();

    for item in iter {
        if item.is_empty() {
            monke_strings.push(monke.clone());
            monke.clear();
            continue;
        }
        monke.push(item);
    }
    monke_strings.push(monke.clone());

    monke_strings
}
