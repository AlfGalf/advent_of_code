use regex::Regex;
use std::{error::Error, fs, usize};

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("inputs/day3.txt")?;
    let mut lines = contents.lines();

    let mut last_line: Option<&str> = None;
    let mut current_line: Option<&str> = lines.next();
    let mut next_line: Option<&str> = lines.next();

    let mut result = 0;
    let num_regex = Regex::new(r"\d+").unwrap();
    let symbol_regex = Regex::new(r"[^\d\.]").unwrap();
    while let Some(cur_line) = current_line {
        for c in num_regex.captures_iter(cur_line) {
            let capture = c.get(0).unwrap();
            let start = capture.start();
            let end = capture.end();

            if 'cond: {
                if let Some(last_line) = last_line {
                    let section = &last_line[(start.max(1) - 1)..(end + 1).min(last_line.len())];
                    if symbol_regex.is_match(section) {
                        break 'cond true;
                    }
                }
                let section = &cur_line[(start.max(1) - 1)..(end + 1).min(cur_line.len())];
                if symbol_regex.is_match(section) {
                    break 'cond true;
                }
                if let Some(next_line) = next_line {
                    let section = &next_line[(start.max(1) - 1)..(end + 1).min(next_line.len())];
                    if symbol_regex.is_match(section) {
                        break 'cond true;
                    }
                }
                false
            } {
                // println!("{}", capture.as_str());
                result += capture.as_str().parse::<usize>().unwrap()
            }
        }

        last_line = current_line;
        current_line = next_line;
        next_line = lines.next();
    }
    println!("{}", result);

    Ok(())
}
