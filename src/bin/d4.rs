use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Game {
    winning_numbers: HashSet<u64>,
    played_numbers: HashSet<u64>
}

impl Game {
    fn matches(&self) -> u64 {
        let matches = self.played_numbers.intersection(&self.winning_numbers).count();
        if matches > 0 {
            return 2_u64.pow((matches - 1) as u32);
        }
        0
    }
}

fn process_line(line: &str) -> Game {
    let all_numbers = line.split(':').nth(1).expect("Invalid line");
    let all_numbers_split: Vec<_> = all_numbers.split('|').collect();
    let winning_numbers: HashSet<u64> = HashSet::from_iter(
        all_numbers_split[0]
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
        );
    let played_numbers: HashSet<u64> = HashSet::from_iter(
        all_numbers_split[1]
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
        );

    Game { winning_numbers, played_numbers }
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let all_parts = args.len() < 3;

    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut sum = 0;
    loop {
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            break;
        }
        let game = process_line(&line.trim());
        sum += game.matches();
        line.clear()
    }
    println!("{}", sum);
    Ok(())
}