use regex::Regex;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("inputs/day1.txt")?;

    let regex_first = Regex::new(r"[^\d]*(\d).*$")?;
    let regex_last = Regex::new(r".*(\d)[^\d]*$")?;

    let val: usize = contents
        .lines()
        .map(|l| {
            println!("{l}");

            let m_first = regex_first.captures(l).unwrap();
            let m_last = regex_last.captures(l).unwrap();

            let (d1, d2) = (
                m_first.get(1).unwrap().as_str(),
                m_last.get(1).unwrap().as_str(),
            );

            println!("{d1} {d2}");
            let str: String = d1.to_string() + d2;

            let val: usize = str.parse().unwrap();

            val
        })
        .sum();

    println!("{}", val);

    Ok(())
}
