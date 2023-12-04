use regex::Regex;
use std::{fs, error::Error, usize};

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("inputs/day4.txt")?;
    let mut lines = contents.lines();

    let card_regex = Regex::new(r"Card[ \d]+:([\d ]+)\|([\d ]+)").unwrap();
    let num_regex = Regex::new(r"\d+").unwrap();

    let res: usize = lines.map(|line| {
        println!("{}", line);
        let reg = card_regex.captures(line).unwrap();
        let winning = reg.get(1).unwrap();
        let numbers_yh = reg.get(2).unwrap();

        let winning: Vec<usize> = num_regex.captures_iter(winning.as_str()).map(|n| n.get(0).unwrap().as_str().parse().unwrap()).collect();
        let numbers_yh: Vec<usize> = num_regex.captures_iter(numbers_yh.as_str()).map(|n| n.get(0).unwrap().as_str().parse().unwrap()).collect();

        let res = numbers_yh.iter().filter(|n| winning.contains(n)).count();

        if res == 0 {
            0
        } else {
            1 << (res - 1)
        }
    }).sum();

    println!("{}", res);

    Ok(())
}