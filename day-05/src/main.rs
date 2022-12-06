use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

#[derive(Debug)]
struct CraneInstructions {
    amount: u32,
    from: usize,
    to: usize,
}

impl TryFrom<&str> for CraneInstructions {
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let words: Vec<&str> = value.split_whitespace().collect();
        let amount: u32 = words
            .get(1)
            .ok_or("Missing amount")?
            .parse::<u32>()
            .expect("Bad amount");
        let from = words
            .get(3)
            .ok_or("Missing from")?
            .parse::<usize>()
            .expect("Bad from")
            - 1;
        let to = words
            .get(5)
            .ok_or("Missing to")?
            .parse::<usize>()
            .expect("Bad to")
            - 1;

        Ok(CraneInstructions {
            amount: amount,
            from: from,
            to: to,
        })
    }
    type Error = &'static str;
}

#[derive(Debug)]
struct Ship {
    crate_stacks: VecDeque<Vec<String>>,
}

trait Crane {
    fn move_crate(&mut self, instruction: &CraneInstructions);
    fn move_crate2(&mut self, instruction: &CraneInstructions);

    fn move_crane(&mut self, instructions: VecDeque<CraneInstructions>);
    fn move_crane2(&mut self, instructions: VecDeque<CraneInstructions>);

    fn grab_tops(&mut self) -> Vec<String>;
}

impl Crane for Ship {
    fn move_crate(&mut self, instruction: &CraneInstructions) {
        let mut grip: Vec<String> = Vec::new();
        for _ in 0..instruction.amount {
            grip.push(self.crate_stacks[instruction.from].pop().unwrap())
        }
        self.crate_stacks[instruction.to].append(&mut grip);
    }

    fn move_crate2(&mut self, instruction: &CraneInstructions) {
        let mut grip: Vec<String> = Vec::new();
        for _ in 0..instruction.amount {
            grip.push(self.crate_stacks[instruction.from].pop().unwrap())
        }
        grip.reverse();
        self.crate_stacks[instruction.to].append(&mut grip);
    }

    fn move_crane(&mut self, instructions: VecDeque<CraneInstructions>) {
        instructions
            .iter()
            .map(|v| self.move_crate(v))
            .for_each(drop);
    }

    fn move_crane2(&mut self, instructions: VecDeque<CraneInstructions>) {
        instructions
            .iter()
            .map(|v| self.move_crate2(v))
            .for_each(drop);
    }

    fn grab_tops(&mut self) -> Vec<String> {
        let tops: Vec<String> = self
            .crate_stacks
            .iter()
            .map(|v| v.last().unwrap_or(&" ".to_string()).to_string())
            .collect();
        tops
    }
}

fn build_ship() -> Result<(Ship, VecDeque<CraneInstructions>), ParseIntError> {
    let file = File::open("./data/input").expect("Couldn't file");
    let mut buf: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
    buf.reverse();

    let mut iterator = buf.iter();
    let mut instructions: VecDeque<CraneInstructions> = VecDeque::new();
    let mut maybe_line = iterator.next();
    while maybe_line.is_some() {
        let line = maybe_line.unwrap();
        if line.is_empty() {
            maybe_line = iterator.next();
            continue;
        }
        if !line.starts_with("move") {
            break;
        }
        match line.as_str().try_into() {
            Ok(value) => instructions.push_front(value),
            Err(_) => break,
        }
        maybe_line = iterator.next();
    }

    let crate_amount: usize = maybe_line
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()?;

    maybe_line = iterator.next();

    let mut crate_stacks: VecDeque<Vec<String>> = VecDeque::new();

    for _ in 0..crate_amount {
        crate_stacks.push_back(Vec::new())
    }

    while maybe_line.is_some() {
        let line = maybe_line.unwrap();

        let mut crate_num = 0;
        let chars: Vec<char> = line.chars().collect();
        let mut crate_name: Vec<char> = Vec::new();
        for (idx, element) in chars.iter().enumerate() {
            if ((idx + 1) % 4) == 0 && idx != 0 {
                if !crate_name[1].is_whitespace() {
                    crate_stacks[crate_num].push(crate_name[1].to_string());
                }
                crate_num += 1;
                crate_name.clear();
                continue;
            }
            crate_name.push(element.to_owned());
        }
        if !crate_name[1].is_whitespace() {
            crate_stacks[crate_num].push(crate_name[1].to_string());
            crate_name.clear();
        }

        maybe_line = iterator.next();
    }

    Ok((
        Ship {
            crate_stacks: crate_stacks,
        },
        instructions,
    ))
}

fn main() {
    let (mut ship, mut instructions) = build_ship().expect("AAAAAAAA");

    println!("Craning...");
    ship.move_crane(instructions);
    println!("\tCraned ship\n{:?}", ship.grab_tops());

    (ship, instructions) = build_ship().expect("AAAAAAAA");
    println!("Craning2...");
    ship.move_crane2(instructions);
    println!("\tCraned ship\n{:?}", ship.grab_tops());
}
