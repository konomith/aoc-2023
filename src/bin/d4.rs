use std::collections::{HashSet, HashMap};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Game {
    winning_numbers: HashSet<u64>,
    played_numbers: HashSet<u64>
}

impl Game {
    fn matches(&self) -> u64 {
        self.played_numbers.intersection(&self.winning_numbers).count() as u64
    }
    fn matches_part1(&self) -> u64 {
        let matches = self.matches();
        if matches > 0 {
            return 2_u64.pow((matches - 1) as u32);
        }
        0
    }
}

#[derive(Debug)]
struct CopyTracker {
    copies: u64
}

impl CopyTracker {
    fn new() -> Self {
        CopyTracker { copies: 1 }
    }
}

struct GameTracker {
    current_id: u64,
    games: HashMap<u64, CopyTracker>
}

impl GameTracker {
    fn new() -> Self {
        GameTracker { current_id: 1, games: HashMap::new() }
    }

    fn track(&mut self, game: &Game) {
        let matches = game.matches();
        let copies = self.games.entry(self.current_id).or_insert(CopyTracker::new()).copies;
        for i in self.current_id + 1..self.current_id + matches + 1 {
            let curr_entry = self.games.entry(i).or_insert(CopyTracker::new());
            curr_entry.copies += copies;
        }
        self.current_id += 1;
    } 

    fn solve(&self) -> u64 {
        let mut sum = 0;
        for point_tracker in self.games.values() {
            sum += point_tracker.copies;
        }
        sum
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

    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut sum = 0;
    let mut tracker = GameTracker::new();
    loop {
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            break;
        }
        let game = process_line(&line.trim());
        sum += game.matches_part1();
        tracker.track(&game);
        line.clear()
    }
    println!("{}", sum);
    println!("{}", tracker.solve());
    Ok(())
}