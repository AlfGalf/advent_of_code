use std::{fs, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let binding = fs::read_to_string("inputs/day13.txt")?;

    let maps: Vec<Vec<Vec<char>>> = binding.split("\n\n").map( |l|
        l.lines().map(|l| l.chars().collect()).collect()
    ).collect();

    let res: usize = maps.iter().map(|l|{

        for x in 1..l[0].len() {
            if l.iter().map(|l| {
                let len = x.min(l.len() - x);

                let left = l[x-len..x].iter().rev();
                let right = l[x..x + len].iter();
                println!("{:?} {:?}", left, right);

                left.zip(right).filter(|(l, r)| l != r).count()
            }).sum::<usize>() == 1 {
                return x;
            }
        }
        for y in 1..l.len() {
            if (0..l[0].len()).map(|x | {
                let arr: Vec<char> = l.iter().map(|l| l[x]).collect();

                let len = y.min(arr.len() - y);
                let left = arr[y-len..y].iter().rev();
                let right = arr[y..y + len].iter();

                left.zip(right).filter(|(l, r)| l != r).count()
            }).sum::<usize>() == 1 {
                return y * 100;
            }
        }
        panic!("Failed to find symetry?\n{:?}", l);
    }).sum();

    println!("{res}");

    Ok(())
}