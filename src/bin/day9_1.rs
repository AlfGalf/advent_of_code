use regex::Regex;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let binding = fs::read_to_string("inputs/day9.txt")?;

    let instructions = Regex::new(r"-?\d+").unwrap();

    let instructions: Vec<Vec<isize>> = binding
        .lines()
        .map(|l| {
            instructions
                .captures_iter(l)
                .map(|c| c.get(0).unwrap().as_str().parse().unwrap())
                .collect()
        })
        .collect();

    let val: isize = instructions
        .into_iter()
        .map(|i| {
            let mut arrs: Vec<Vec<isize>> = vec![i];
            loop {
                if arrs.last().unwrap().windows(2).all(|w| w[1] == w[0]) {
                    break;
                }
                arrs.push(
                    arrs.last()
                        .unwrap()
                        .windows(2)
                        .map(|w| w[1] - w[0])
                        .collect(),
                );
            }

            println!("{arrs:?}");

            let val = *arrs.last().unwrap().first().unwrap();
            arrs.last_mut().unwrap().push(val);

            for i in (0..arrs.len() - 1).rev() {
                let val = arrs[i].last().unwrap() + arrs[i + 1].last().unwrap();
                arrs[i].push(val);
            }
            println!("{arrs:?}");
            *arrs.first().unwrap().last().unwrap()
        })
        .sum();

    println!("{val}");

    Ok(())
}
