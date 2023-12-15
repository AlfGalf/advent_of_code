use regex::Regex;
use std::{collections::HashMap, error::Error, fs, usize};

fn map(c1: char) -> usize {
    match c1 {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let binding = fs::read_to_string("inputs/day7.txt")?;

    let card = Regex::new(r"(?m)([AKQJT2-9]{5}) (\d+)").unwrap();

    let cards: Vec<(String, usize)> = card
        .captures_iter(&binding)
        .map(|c| {
            (
                c.get(1).unwrap().as_str().to_string(),
                c.get(2).unwrap().as_str().parse().unwrap(),
            )
        })
        .collect();

    let mut vals = cards
        .iter()
        .map(|(c, v)| {
            let mut cards: HashMap<char, usize> = c.chars().fold(HashMap::new(), |mut hm, c| {
                hm.entry(c).and_modify(|v| *v += 1).or_insert(1);
                hm
            });

            let most = cards.iter().max_by(|l, r| l.1.cmp(r.1)).unwrap();
            let most = (*most.0, *most.1);

            cards.insert(most.0, 0);

            let second_most = cards.iter().max_by(|l, r| l.1.cmp(r.1)).unwrap().clone();

            let second_most = (*second_most.0, *second_most.1);

            (c.clone(), most, second_most, *v)
        })
        .collect::<Vec<_>>();

    println!("{:?}", vals);
    vals.sort_by(|c1, c2| {
        if c1.1 .1 != c2.1 .1 {
            return c1.1 .1.cmp(&c2.1 .1);
        };
        if c1.2 .1 != c2.2 .1 {
            return c1.2 .1.cmp(&c2.2 .1);
        }
        for i in 0..5 {
            let l_char = c1.0.chars().nth(i).unwrap();
            let r_char = c2.0.chars().nth(i).unwrap();
            if l_char != r_char {
                return map(l_char).cmp(&map(r_char));
            }
        }
        panic!("No ordering!")
    });
    // println!("{:?}", vals);

    let val: usize = vals
        .iter()
        .enumerate()
        .map(|(r, c)| {
            println!("{} {} {}", c.0, c.3, r + 1);
            c.3 * (r + 1)
        })
        .sum();

    println!("{val}");

    Ok(())
}
