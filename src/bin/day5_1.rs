use regex::Regex;
use std::{fs, error::Error, usize, collections::HashMap};

fn lookup(dict: &Vec<(usize, usize, usize)>, val: usize) -> usize {
    for (dest, source, range) in dict {
        if source <= &val && val < source + range {
            return dest + (val - source);
        }
    }
    val
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("inputs/day5.txt")?;

    let seeds_regex = Regex::new(r"(?m)^seeds: ([\d ]+)$").unwrap();
    let dict_regex = Regex::new(r"(?m)^([\w-]+) map:\n(([\d ]+\n)+)$").unwrap();

    let entry_regex = Regex::new(r"(?m)(\d+) (\d+) (\d+)").unwrap();

    let num_regex = Regex::new(r"\d+").unwrap();

    let seeds = seeds_regex.captures(&contents).unwrap().get(1).unwrap().as_str();
    let seeds: Vec<usize> = num_regex.captures_iter(seeds).map(|c| c.get(0).unwrap().as_str().parse().unwrap()).collect();

    let dicts: HashMap<String, Vec<(usize, usize, usize)>> = dict_regex.captures_iter(&contents).map(|c| {
        let name = c.get(1).unwrap().as_str().to_string();
        let entries = c.get(2).unwrap().as_str();
        let entries = entry_regex.captures_iter(entries).map(|c|
            (c.get(1).unwrap().as_str().parse().unwrap(), c.get(2).unwrap().as_str().parse().unwrap(), c.get(3).unwrap().as_str().parse().unwrap())
        ).collect();
        (name, entries)
    }).collect();

    // println!("{:?}", seeds);
    // println!("{:?}", dicts);

    let val = seeds.iter().map(|s| {
        let soil = lookup(&dicts["seed-to-soil"], *s);
        let fertilizer = lookup(&dicts["soil-to-fertilizer"], soil);
        let water= lookup(&dicts["fertilizer-to-water"], fertilizer);
        let light = lookup(&dicts["water-to-light"], water);
        let temp = lookup(&dicts["light-to-temperature"], light);
        let humidity = lookup(&dicts["temperature-to-humidity"], temp);
        let location= lookup(&dicts["humidity-to-location"], humidity);
        location
    }).min().unwrap();

    println!("{val}");

    Ok(())
}