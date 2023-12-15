use regex::Regex;
use std::{collections::HashMap, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let binding = fs::read_to_string("inputs/day8.txt")?;

    let first_line = binding.lines().next().unwrap().to_string();

    let instructions = Regex::new(r"(?m)^([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)$").unwrap();

    let instructions: HashMap<String, (String, String)> = instructions
        .captures_iter(&binding)
        .map(|c| {
            (
                c.get(1).unwrap().as_str().to_string(),
                (
                    c.get(2).unwrap().as_str().to_string(),
                    c.get(3).unwrap().as_str().to_string(),
                ),
            )
        })
        .collect();

    let mut i = 0;
    let mut cur = "AAA";
    while cur != "ZZZ" {
        for char in first_line.chars() {
            i += 1;
            println!("{cur}");
            if cur == "ZZZ" {
                break;
            }
            if char == 'L' {
                cur = &instructions.get(cur).unwrap().0
            } else if char == 'R' {
                cur = &instructions.get(cur).unwrap().1
            } else {
                panic!("Unknown symbol")
            }
        }
    }

    println!("{i}");

    Ok(())
}
