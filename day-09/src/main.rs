use std::{
    cmp::max,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("can't resolve"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Move {
    direction: Direction,
    value: i32,
}

impl From<String> for Move {
    fn from(value: String) -> Self {
        let mut command = value.split_whitespace();
        Move {
            direction: command.next().unwrap().into(),
            value: command.next().unwrap().parse().expect("NaN"),
        }
    }
}

impl From<Move> for Position {
    fn from(move_val: Move) -> Self {
        match move_val.direction {
            Direction::Right => Self {
                x: move_val.value,
                y: 0,
            },
            Direction::Left => Self {
                x: -move_val.value,
                y: 0,
            },
            Direction::Up => Self {
                x: 0,
                y: move_val.value,
            },
            Direction::Down => Self {
                x: 0,
                y: -move_val.value,
            },
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Position {
    fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    fn set_value(&mut self, value: Position) {
        self.x = value.x;
        self.y = value.y;
    }
}

impl Move {
    fn get_path(&self) -> Vec<Position> {
        let mut path = Vec::new();
        for _ in 1..self.value + 1 {
            path.push(
                Move {
                    direction: self.direction,
                    value: 1,
                }
                .into(),
            )
        }
        path
    }
}

trait Moves {
    fn execute_moves(&mut self, moves: Vec<Move>);
}

struct Rope {
    knots: Vec<Position>,
    tail_positions: HashSet<Position>,
}

impl Rope {
    fn new(knots: Vec<Position>) -> Rope {
        Rope {
            knots: knots,
            tail_positions: HashSet::from([Position::new()]),
        }
    }

    fn process_step(&mut self, step: Position) {
        self.process_head(step);
        self.process_tail();
        self.tail_positions
            .insert(self.knots.last().unwrap().clone());
    }

    fn process_head(&mut self, step: Position) {
        let head = self.knots.get_mut(0).unwrap();
        head.set_value(*head + step);
    }

    fn process_tail(&mut self) {
        for idx in 1..self.knots.len() {
            let diff =
                *self.knots.get(idx - 1).clone().unwrap() - *self.knots.get(idx).clone().unwrap();
            if max(diff.x.abs(), diff.y.abs()) <= 1 {
                continue;
            }
            let knot = self.knots.get_mut(idx).unwrap();
            let step_value = Position {
                x: diff.x.checked_div(diff.x.abs()).unwrap_or(0),
                y: diff.y.checked_div(diff.y.abs()).unwrap_or(0),
            };
            knot.set_value(*knot + step_value);
        }
    }

    fn execute_moves(&mut self, moves: Vec<Move>) {
        for move_val in moves {
            for step in move_val.get_path() {
                self.process_step(step);
            }
        }
    }
}

fn load_data(path: &str) -> Vec<Move> {
    let file = File::open(path).expect("Can't open");
    let buf = BufReader::new(file);

    let mut moves: Vec<Move> = Vec::new();
    for line in buf.lines().map(|l| l.expect("Parsing error")) {
        moves.push(line.into())
    }

    moves
}

fn main() {
    let mut rope1 = Rope::new(vec![Position::new(); 2]);
    let moves = load_data("./data/input");
    rope1.execute_moves(moves.clone());
    println!("Solution1: {:?}", rope1.tail_positions.len());

    let mut rope2 = Rope::new(vec![Position::new(); 10]);
    rope2.execute_moves(moves);
    println!("Solution2: {:?}", rope2.tail_positions.len());
}
