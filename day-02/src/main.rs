use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

mod rps;
use log::info;
use rps::{FixedGame, Game, Match};

static STRATEGY_FILE: &'static str = "./data/1.txt";

fn load_strategies(path: &str) -> Vec<String> {
    let file_path = Path::new(path);
    let file = File::open(file_path).expect("Couldn't open file");

    let buf = BufReader::new(file);
    buf.lines().map(|l| l.unwrap()).collect()
}

fn solve() {
    let strategies = load_strategies(STRATEGY_FILE);
    let mut base_result = 0;
    let mut fixed_result = 0;
    for strategy in strategies {
        let game: Game = strategy.as_str().into();
        let fixed_game: FixedGame = strategy.as_str().into();
        base_result += game.result();
        fixed_result += fixed_game.result()
    }
    info!("Base game: {:?}", base_result);
    info!("Fixed game: {:?}", fixed_result);
}

fn main() {
    ::std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    solve();
}
