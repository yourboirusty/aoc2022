use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

struct Circuit {
    register: i32,
    clock: u32,
    signal_checker: u32,
    signal_step: u32,

    crt_position: i32,
    crt_width: i32,
    crt_height: usize,
    crt_row: String,
    crt_display: String,

    crt_empty_char: char,
    crt_full_char: char,

    instructions: VecDeque<Option<i32>>,
}

impl Circuit {
    fn new() -> Self {
        Circuit {
            register: 1,
            clock: 0,

            signal_checker: 20,
            signal_step: 40,

            crt_position: 0,
            crt_width: 40,
            crt_height: 6,
            crt_row: "".to_string(),
            crt_display: "".to_string(),

            crt_empty_char: ' ',
            crt_full_char: '█',

            instructions: VecDeque::new(),
        }
    }

    fn add(&mut self, val: i32) {
        self.instructions
            .append(&mut VecDeque::from([None, Some(val)]));
    }

    fn draw_crt(&mut self) {
        let sprite_range = (self.register - 1)..(self.register + 2);
        if sprite_range.contains(&(self.crt_position)) {
            self.crt_row.push(self.crt_full_char);
        } else {
            self.crt_row.push(self.crt_empty_char);
        }
        self.crt_position += 1;
        if self.crt_position >= self.crt_width {
            self.crt_display.push_str(&self.crt_row.as_str());
            self.crt_display.push('\n');
            self.crt_row = "".to_string();
            self.crt_position = 0;
        }
    }

    fn noop(&mut self) {
        self.instructions.push_back(None);
    }

    fn signal_strength(&self) -> i32 {
        self.register * self.clock as i32
    }

    fn tick(&mut self) {
        let instruction = self.instructions.pop_front();
        match instruction.unwrap() {
            None => (),
            Some(val) => self.register += val,
        }
    }

    fn check_signal(&mut self) -> i32 {
        if self.clock < self.signal_checker {
            return 0;
        }
        self.signal_checker += self.signal_step;
        let signal_strength = self.signal_strength();
        signal_strength
    }

    fn solve(&mut self) -> i32 {
        let mut signal_strenght_sum = 0;
        for _ in 0..self.instructions.len() {
            self.clock += 1;
            self.draw_crt();
            signal_strenght_sum += self.check_signal();
            self.tick();
        }
        signal_strenght_sum
    }
}

fn load_data(path: &str) -> Circuit {
    let file = File::open(path).expect("Can't open");
    let buf = BufReader::new(file);

    let mut circuit = Circuit::new();
    for line in buf.lines().map(|l| l.expect("Parsing err")) {
        let mut command = line.split_whitespace();
        match command.next().unwrap() {
            "noop" => circuit.noop(),
            "addx" => circuit.add(command.next().unwrap().parse().expect("NaN")),
            _ => panic!("unknown command"),
        }
    }
    circuit
}

fn main() {
    let mut circuit = load_data("./data/input");
    println!("{:?}", circuit.solve());
    print!("{}", circuit.crt_display);
}
