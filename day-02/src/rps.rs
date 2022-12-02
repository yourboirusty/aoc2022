use core::panic;
use log::debug;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock = 1,
    Paper,
    Scissors,
}

enum Result {
    Loss = 0,
    Draw = 3,
    Win = 6,
}
pub struct Game {
    player: RPS,
    opponent: RPS,
}

pub struct FixedGame {
    opponent: RPS,
    result: Result,
}

pub trait Match {
    fn result(&self) -> u32;
}

trait Mechanic {
    fn loses_to(&self) -> RPS;
    fn wins_against(&self) -> RPS;
}

trait FixMatch {
    fn fix_match(&self) -> RPS;
}

impl Mechanic for RPS {
    fn loses_to(&self) -> RPS {
        let code = (*self as i32 % 3) + 1;
        RPS::from(code)
    }
    fn wins_against(&self) -> RPS {
        let code = ((*self as u32 + 1) % 3) + 1;
        RPS::from(code)
    }
}

impl From<&str> for RPS {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            _ => panic!("Unknown move"),
        }
    }
}

impl From<&str> for Result {
    fn from(value: &str) -> Self {
        match value {
            "X" => Result::Loss,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            _ => panic!("Unknown result"),
        }
    }
}

impl From<u32> for RPS {
    fn from(value: u32) -> Self {
        match value {
            1 => RPS::Rock,
            2 => RPS::Paper,
            3 => RPS::Scissors,
            _ => panic!("No such rps"),
        }
    }
}

impl From<i32> for RPS {
    fn from(value: i32) -> Self {
        match value {
            1 => RPS::Rock,
            2 => RPS::Paper,
            3 => RPS::Scissors,
            _ => panic!("No such rps"),
        }
    }
}

impl PartialEq for RPS {
    fn eq(&self, other: &Self) -> bool {
        *self as u32 == *other as u32
    }
}

impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.wins_against() == *other {
            debug!("{:?} wins over {:?}", *self, *other);
            return Some(Ordering::Greater);
        } else if self == other {
            debug!("{:?} draws {:?}", *self, *other);
            return Some(Ordering::Equal);
        }
        debug!("{:?} loses over {:?}", *self, *other);
        Some(Ordering::Less)
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let moves = value.split(" ").collect::<Vec<&str>>();
        if moves.len() != 2 {
            panic!("Invalid number of moves");
        }

        Game {
            opponent: moves[0].into(),
            player: moves[1].into(),
        }
    }
}

impl From<&str> for FixedGame {
    fn from(value: &str) -> Self {
        let entry = value.split(" ").collect::<Vec<&str>>();

        FixedGame {
            opponent: entry[0].into(),
            result: entry[1].into(),
        }
    }
}

impl Match for Game {
    fn result(&self) -> u32 {
        let mut result = self.player as u32;
        debug!("Points for sign: {:?}", result);
        if self.player > self.opponent {
            result += Result::Win as u32;
        } else if self.player == self.opponent {
            result += Result::Draw as u32;
        } else {
            result += Result::Loss as u32;
        }
        debug!("Score: {:?}", result);
        result
    }
}

impl FixMatch for FixedGame {
    fn fix_match(&self) -> RPS {
        match self.result {
            Result::Draw => return self.opponent,
            Result::Loss => return self.opponent.wins_against(),
            Result::Win => return self.opponent.loses_to(),
        }
    }
}

impl Match for FixedGame {
    fn result(&self) -> u32 {
        Game {
            opponent: self.opponent,
            player: self.fix_match(),
        }
        .result()
    }
}
