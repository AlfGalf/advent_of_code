use std::{error::Error, fs};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let binding = fs::read_to_string("inputs/day15.txt")?;

    let inputs: Vec<&str> = binding.lines().next().unwrap().split(',').collect_vec();

    let res: usize = inputs
        .iter()
        .map(|s| {
            s.bytes()
                .fold(0, |acc, c| u8::wrapping_mul(u8::wrapping_add(acc, c), 17))
                as usize
        })
        .sum();

    println!("{res}");

    Ok(())
}
