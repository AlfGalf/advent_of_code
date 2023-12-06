use regex::Regex;
use std::{fs, error::Error, usize, collections::HashMap, mem::Discriminant};
use rayon::prelude::*;

fn lookup(dict: &Vec<(usize, usize, usize)>, val: usize) -> usize {
    for (dest, source, range) in dict {
        if source <= &val && val < source + range {
            return dest + (val - source);
        }
    }
    val
}

fn main() -> Result<(), Box<dyn Error>> {
    let binding = fs::read_to_string("inputs/day6.txt")?;
    let mut contents = binding.lines();

    let num_regex = Regex::new(r"\d+").unwrap();
    let times: String = (&mut contents).next().unwrap().chars().filter(|c| !c.is_whitespace()).collect();
    let distances: String = (&mut contents).next().unwrap().chars().filter(|c| !c.is_whitespace()).collect();
    let times: Vec<usize> = num_regex.captures_iter(&times).map(|c| c.get(0).unwrap().as_str().parse().unwrap()).collect();
    let distances: Vec<usize> = num_regex.captures_iter(&distances).map(|c| c.get(0).unwrap().as_str().parse().unwrap()).collect();

    let val: usize = times.iter().zip(distances.iter()).map(|(t, d)| {
        (1..*t).filter(|ti| ti * (t - ti) > *d).count()
    }).product();

    println!("{val}");

    Ok(())
}