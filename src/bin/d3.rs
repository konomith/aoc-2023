use std::collections::HashMap;
use std::env;

use std::fs::File;

use std::io::{BufRead, BufReader};

struct Schema {
    grid: Vec<Vec<char>>
}

impl Schema {
    fn new() -> Self {
        Schema { grid: Vec::new() }
    }

    fn add_row(&mut self, row: &str) {
        self.grid.push(row.chars().collect());
    }

    fn is_part(&self, r: i64, c: i64) -> bool {
        let max_row = self.grid.len() as i64;
        let max_col = self.grid[r as usize].len() as i64;
        let offsets_to_check = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
        for (x, y) in offsets_to_check {
            if r + x >= 0 && r + x < max_row && c + y >= 0 && c + y < max_col {
                let to_check = self.grid[(r + x) as usize][(c + y) as usize];
                if !to_check.is_digit(10) && !(to_check == '.') {
                    return true;
                }
            }
        }
        false
    }

    fn solve(&self) -> u64 {
        let mut sum: u64 = 0;
        for (r, row) in self.grid.iter().enumerate() {
            let mut num_as_string = String::new();
            let mut is_part = false;
            for (c, chr) in row.iter().enumerate() {
                if chr.is_digit(10) {
                    num_as_string.push(*chr);
                    if self.is_part(r as i64, c as i64) {
                        is_part = true;
                    }
                } else {
                    if !num_as_string.is_empty() && is_part {
                        let num = num_as_string.parse::<u64>().unwrap();
                        sum += num;
                    }
                    num_as_string.clear();
                    is_part = false;
                }
            }

            if !num_as_string.is_empty() && is_part {
                let num = num_as_string.parse::<u64>().unwrap();
                sum += num;
            }
        }
        sum
    }

    fn is_gear_part(&self, r: i64, c: i64) -> (bool, i64, i64) {
        let max_row = self.grid.len() as i64;
        let max_col = self.grid[r as usize].len() as i64;
        let offsets_to_check = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
        for (x, y) in offsets_to_check {
            if r + x >= 0 && r + x < max_row && c + y >= 0 && c + y < max_col {
                let to_check = self.grid[(r + x) as usize][(c + y) as usize];
                if to_check == '*' {
                    return (true, r + x, c + y);
                }
            }
        }
        (false, 0, 0)
    }

    fn solve_p2(&self) -> u64 {
        let mut sum: u64 = 0;
        let mut map = HashMap::new();

        for (r, row) in self.grid.iter().enumerate() {
            let mut num_as_string = String::new();
            let mut is_gear_part = false;
            let mut possible_coords = (-1, -1);
            for (c, chr) in row.iter().enumerate() {
                if chr.is_digit(10) {
                    num_as_string.push(*chr);
                    let (is_gear, i, j) = self.is_gear_part(r as i64, c as i64);
                    if is_gear {
                        is_gear_part = true;
                        possible_coords = (i, j);
                    }
                } else {
                    if !num_as_string.is_empty() && is_gear_part {
                        let num = num_as_string.parse::<u64>().unwrap();
                        map.entry(possible_coords).or_insert(Vec::new()).push(num);
                    }
                    num_as_string.clear();
                    is_gear_part = false;
                }
            }

            if !num_as_string.is_empty() && is_gear_part {
                let num = num_as_string.parse::<u64>().unwrap();
                map.entry(possible_coords).or_insert(Vec::new()).push(num);
            }
        }

        for val in map.values() {
            if val.len() > 1 {
                sum += val.iter().product::<u64>();
            }
        }
        sum
    }
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let all_parts = args.len() < 3;

    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut schema = Schema::new();
    loop {
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            break;
        }
        if line.len() > 0 {
            schema.add_row(&line.trim());
        }
        line.clear()
    }

    println!("{}", schema.solve());
    if all_parts {
        println!("{}", schema.solve_p2());
    }
    Ok(())
}
