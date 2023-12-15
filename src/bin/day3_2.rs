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
    let symbol_regex = Regex::new(r"(\d*)(\*)(\d*)").unwrap();
    while let Some(cur_line) = current_line {
        for c in symbol_regex.captures_iter(cur_line) {
            let capture = c.get(2).unwrap();
            let pos = capture.start();

            let mut nums = vec![];

            if let Some(last_line) = last_line {
                for num in num_regex.captures_iter(last_line) {
                    let num_cap = num.get(0).unwrap();
                    let num_int: usize = num_cap.as_str().parse().unwrap();
                    if num_cap.end() >= pos && num_cap.start() <= pos + 1 {
                        nums.push(num_int)
                    }
                }
            }
            if let Ok(left_num) = c.get(1).unwrap().as_str().parse::<usize>() {
                nums.push(left_num)
            }
            if let Ok(right_num) = c.get(3).unwrap().as_str().parse::<usize>() {
                nums.push(right_num)
            }
            if let Some(next_line) = next_line {
                for num in num_regex.captures_iter(next_line) {
                    let num_cap = num.get(0).unwrap();
                    let num_int: usize = num_cap.as_str().parse().unwrap();
                    if num_cap.end() >= pos && num_cap.start() <= pos + 1 {
                        nums.push(num_int)
                    }
                }
            }

            if nums.len() == 2 {
                result += nums[0] * nums[1]
            }
        }

        last_line = current_line;
        current_line = next_line;
        next_line = lines.next();
    }
    println!("{}", result);

    Ok(())
}
