use lazy_static::lazy_static;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

lazy_static! {
    static ref GAME_ID: Regex = Regex::new(r"Game (\d+)").unwrap();
    static ref RED: Regex = Regex::new(r"(\d+) red").unwrap();
    static ref BLUE: Regex = Regex::new(r"(\d+) blue").unwrap();
    static ref GREEN: Regex = Regex::new(r"(\d+) green").unwrap();
}

#[derive(Debug)]
struct Game {
    id: u64,
    red: u64,
    blue: u64,
    green: u64
}

impl Game {
    fn from_line(line: &str) -> Self {
        let id = get_id(line);
        let red = get_red(line);
        let blue = get_blue(line);
        let green = get_green(line);

        Self { id, red, blue, green }
    }
}

fn get_id(line: &str) -> u64 {
    let captures = GAME_ID.captures(line);
    captures.unwrap().get(1).unwrap().as_str().parse().unwrap_or(0)
}

fn get_red(line: &str) -> u64 {
    let mut max: u64 = 0;
    for (_, count) in RED.captures_iter(line).map(|c| c.extract::<1>()) {
        let curr = count[0].parse::<u64>().unwrap_or(0);
        if curr > max {
            max = curr;
        }
    }
    max
}

fn get_blue(line: &str) -> u64 {
    let mut max: u64 = 0;
    for (_, count) in BLUE.captures_iter(line).map(|c| c.extract::<1>()) {
        let curr = count[0].parse::<u64>().unwrap_or(0);
        if curr > max {
            max = curr;
        }
    }
    max
}

fn get_green(line: &str) -> u64 {
    let mut max: u64 = 0;
    for (_, count) in GREEN.captures_iter(line).map(|c| c.extract::<1>()) {
        let curr = count[0].parse::<u64>().unwrap_or(0);
        if curr > max {
            max = curr;
        }
    }
    max
}

struct Rule {
    red: u64,
    blue: u64,
    green: u64
}

impl Default for Rule {
    fn default() -> Self { 
        Rule {
            red: 12,
            blue: 14,
            green: 13
        }
    }
}

impl Rule {
    fn is_possible_game(&self, game: &Game) -> bool {
        game.red <= self.red && game.blue <= self.blue && game.green <= self.green
    }
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let all_parts = args.len() < 3;

    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut sum: u64 = 0;
    let mut power: u64 = 0;
    let rule = Rule::default();
    loop {
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            break;
        }
        if line.len() > 0 {
            let game = Game::from_line(&line);
            if rule.is_possible_game(&game) {
                sum += game.id;
            }
            if all_parts {
                power += game.red * game.blue * game.green;
            }
        }
        line.clear()
    }
    println!("sum={}, power={}", sum, power);
    Ok(())
}
