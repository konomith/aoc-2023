use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn process_line(line: &str) -> u32 {
    let mut first: u32 = 0;
    let mut last: u32 = 0;

    for c in line.chars() {
        if c.is_digit(10) {
            let c_digit = c.to_digit(10).unwrap();
            if first == 0 {
                first = c_digit;
            }
            last = c_digit;
        }
    }
    first * 10 + last
}

fn format_line_for_part2(line: &mut String) -> String {
    let map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut indices: Vec<(usize, &str)> = Vec::new();

    for (key, _) in map.iter() {
        let matches: Vec<_> = line.match_indices(key).collect();
        indices.extend(&matches);
    }

    indices.sort();

    let mut formatted = line.clone();

    match indices.first() {
        Some((_index, key)) => {
            let new = format!("{}", map.get(key).unwrap());
            formatted = formatted.replacen(key, &new, 1);
        }
        None => {}
    }

    match indices.last() {
        Some((_index, key)) => {
            formatted = formatted.replacen(key, map.get(key).unwrap(), 1);
        }
        None => {}
    }
    formatted
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let all_parts = args.len() < 3;

    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut sum: u32 = 0;

    let mut line = String::new();

    loop {
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            break;
        }
        if all_parts {
            line = format_line_for_part2(&mut line);
        }
        let num = process_line(&line.trim());
        sum += num;
        line.clear()
    }

    println!("{}", sum);
    Ok(())
}
