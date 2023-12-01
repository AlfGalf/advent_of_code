use regex::Regex;
use std::{fs, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("inputs/day1.txt")?;

    let regex_first = Regex::new(r"(:?(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)|(\d)).*$")?;
    let regex_last = Regex::new(r"^.*(:?(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)|(\d))")?;

    let val: usize = contents.lines().map(|l| {
        // println!("{l}");

        let m_first = regex_first.captures(l).unwrap();
        let m_last = regex_last.captures(l).unwrap();

        let (d1, d2) = (m_first.get(1).unwrap().as_str(), m_last.get(1).unwrap().as_str());

        fn get_val(s: &str) -> usize {
            match s {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven"=> 7,
                "eight" => 8,
                "nine" => 9,
                s => s.parse().unwrap()
            }
        }

        let d1:usize = get_val(d1);
        let d2:usize = get_val(d2);

        // println!("{d1} {d2}");

        let val: usize = d1 * 10 + d2;

        val
    }).sum();

    println!("{}", val);

    Ok(())
}