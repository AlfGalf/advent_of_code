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

    let seed_regex = Regex::new(r"(\d+) (\d+)").unwrap();

    let seeds = seeds_regex.captures(&contents).unwrap().get(1).unwrap().as_str();
    let seeds: Vec<(usize, usize)> = seed_regex.captures_iter(seeds).map(|c| (c.get(1).unwrap().as_str().parse().unwrap(), c.get(2).unwrap().as_str().parse().unwrap())).collect();

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
    let seeds_to_soil = &dicts["seed-to-soil"];
    let soil_to_fert = &dicts["soil-to-fertilizer"];
    let fert_to_water = &dicts["fertilizer-to-water"];
    let water_to_light = &dicts["water-to-light"];
    let light_to_temp = &dicts["light-to-temperature"];
    let temp_to_hum = &dicts["temperature-to-humidity"];
    let hum_to_loc = &dicts["humidity-to-location"];

    let val = seeds.iter().flat_map(|(start, range)| {
        (*start..start + range).map(|s| {
            let soil = lookup(seeds_to_soil, s);
            let fertilizer = lookup(soil_to_fert, soil);
            let water= lookup(fert_to_water, fertilizer);
            let light = lookup(water_to_light, water);
            let temp = lookup(light_to_temp, light);
            let humidity = lookup(temp_to_hum, temp);
            let location= lookup(hum_to_loc, humidity);
            location
        })
    }).min().unwrap();

    println!("{val}");

    Ok(())
}