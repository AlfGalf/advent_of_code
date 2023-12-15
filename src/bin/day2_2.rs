use regex::Regex;
use std::{error::Error, fs, usize};

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("inputs/day2.txt")?;

    let regex_game_and_content = Regex::new(r"^Game (\d+): (.+)$")?;

    let val: usize = contents
        .lines()
        .map(|l| {
            println!("{l}");

            let game_content = regex_game_and_content.captures(l).unwrap();
            let (_, game_content) = (
                game_content
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap(),
                game_content.get(2).unwrap().as_str(),
            );

            let game_parts = game_content.split(";");

            let red_regex = Regex::new(r"(\d+) red").unwrap();
            let red: usize = game_parts
                .clone()
                .map(|s| {
                    red_regex
                        .captures(s)
                        .map_or(0, |m| m.get(1).unwrap().as_str().parse::<usize>().unwrap())
                })
                .max()
                .unwrap_or_default();
            let green_regex = Regex::new(r"(\d+) green").unwrap();
            let green: usize = game_parts
                .clone()
                .map(|s| {
                    green_regex
                        .captures(s)
                        .map_or(0, |m| m.get(1).unwrap().as_str().parse::<usize>().unwrap())
                })
                .max()
                .unwrap_or_default();
            let blue_regex = Regex::new(r"(\d+) blue").unwrap();
            let blue: usize = game_parts
                .clone()
                .map(|s| {
                    blue_regex
                        .captures(s)
                        .map_or(0, |m| m.get(1).unwrap().as_str().parse::<usize>().unwrap())
                })
                .max()
                .unwrap_or_default();
            println!("{red} {green} {blue}");

            red * blue * green
        })
        .sum();

    println!("{}", val);

    Ok(())
}
