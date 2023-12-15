use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let binding = fs::read_to_string("inputs/day13.txt")?;

    let maps: Vec<Vec<Vec<char>>> = binding
        .split("\n\n")
        .map(|l| l.lines().map(|l| l.chars().collect()).collect())
        .collect();

    let res: usize = maps
        .iter()
        .map(|l| {
            for x in 1..l[0].len() {
                if l.iter().all(|l| {
                    let len = x.min(l.len() - x);

                    let mut left = l[x - len..x].to_vec();
                    left.reverse();
                    let right = l[x..x + len].to_vec();
                    println!("{:?} {:?}", left, right);

                    left == right
                }) {
                    return x;
                }
            }
            for y in 1..l.len() {
                if (0..l[0].len()).all(|x| {
                    let arr: Vec<char> = l.iter().map(|l| l[x]).collect();

                    let len = y.min(arr.len() - y);
                    let mut left = arr[y - len..y].to_vec();
                    left.reverse();
                    let right = &arr[y..y + len];

                    left == right
                }) {
                    return y * 100;
                }
            }
            panic!("Failed to find symetry?\n{:?}", l);
        })
        .sum();

    println!("{res}");

    Ok(())
}
