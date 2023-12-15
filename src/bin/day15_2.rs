use arr_macro::arr;
use std::{error::Error, fs, usize};

use itertools::Itertools;
use regex::Regex;

fn hash(s: &str) -> u8 {
    s.bytes()
        .fold(0, |acc, c| u8::wrapping_mul(u8::wrapping_add(acc, c), 17))
}

fn main() -> Result<(), Box<dyn Error>> {
    let binding = fs::read_to_string("inputs/day15.txt")?;

    let inputs: Vec<&str> = binding.lines().next().unwrap().split(',').collect_vec();

    let mut boxes: [Vec<(&str, usize)>; 256] = arr![Default::default(); 256];

    let put_reg = Regex::new(r"(\w+)=(\d+)").unwrap();
    let pop_reg = Regex::new(r"(\w+)-").unwrap();
    for s in inputs {
        if let Some((n, d)) = put_reg.captures(s).map(|c| {
            (
                c.get(1).unwrap().as_str(),
                c.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            )
        }) {
            let t_box = &mut boxes[hash(n) as usize];
            if let Some(b) = t_box.iter_mut().filter(|(tn, _)| &n == tn).next() {
                b.1 = d
            }
            else {
                t_box.push((n, d));
            }
        } else if let Some(n) = pop_reg.captures(s).map(|c| {
            c.get(1).unwrap().as_str()
        }) {
            boxes[hash(n) as usize] = boxes[hash(n) as usize]
                .iter()
                .map(|c| c.clone())
                .filter(|&(tn, _)| tn != n)
                .collect();
        } else {
            panic!("unrecognised command {s}")
        }
    }

    for (n, b) in boxes.iter().enumerate() {
        println!("Box {n}");
        for (ls, ln) in b {
            print!("{ls} {ln},")
        }
        println!();
    }

    let res: usize = boxes
        .into_iter()
        .enumerate()
        .flat_map(|(nb, b)| b.into_iter().enumerate().map(move |(i, (_, d))| (nb + 1) * (i + 1) * d))
        .sum();

    println!("{res}");

    Ok(())
}
