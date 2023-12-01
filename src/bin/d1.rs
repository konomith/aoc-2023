use std::{env, println};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


fn process_line(line: &str) -> i32 {
    let filtered = line
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<Vec<_>>();

    let mut out = String::new();
    match filtered.first() {
        Some(c) => {out.push(*c);}
        None => {}
    }

    match filtered.last() {
        Some(c) => {out.push(*c);}
        None => {}
    }

    match out.parse::<i32>() {
        Ok(r) => {
            r
        }
        Err(_) => {
            0
        }
    }
}

fn format_line(line: &mut String) -> String {
    let map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9")
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
            formatted = formatted.replacen(key, map.get(key).unwrap(), 1);
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
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut sum: i32 = 0;

    let mut line = String::new();

    loop {
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            break;
        }
        line = format_line(&mut line);
        let num = process_line(&line.trim());
        sum += num;
        line.clear()
    }

    println!("{}", sum);
    Ok(())
}
