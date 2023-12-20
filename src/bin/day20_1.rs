use std::{error::Error, fs, usize, collections::{HashMap, VecDeque}};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
enum Module<'a> {
    Broadcast {dests: &'a Vec<&'a str>},
    FlipFlop {dests: &'a Vec<&'a str>, is_on: bool},
    Conjunction {from: Vec<(&'a str, bool)>, to: &'a Vec<&'a str>}
}

impl<'a> Module<'a> {
    fn do_pulse(&mut self, is_high: bool, fname: &str, name: &'a str) -> Vec<(&'a str, bool, &'a str)> {
        match self {
            Module::Broadcast { dests } => dests.iter().map(move |d| (*d, is_high, name)).collect_vec(),
            Module::FlipFlop { dests , is_on } => if !is_high { *is_on = !*is_on; dests.iter().map(move |d: &&str| (*d, *is_on, name)).collect_vec() } else {return vec![]},
            Module::Conjunction { from, to } => {
                from.iter_mut().filter(|(s, _)| s == &fname).for_each(|(_, b)| *b = is_high);

                let is_on = from.iter().any(|(_, b)| !*b);

                to.iter().map(|n| (*n, is_on, name)).collect_vec()
            },
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // let binding = fs::read_to_string("inputs/test.txt")?;
    let binding = fs::read_to_string("inputs/day20.txt")?;

    let modules = Regex::new(r"(?m)^(%|&|)(\w+) -> ([\w, ]+)$")?;

    // let broadcast_dests: Vec<&str> = broadcast.captures(&binding).unwrap().get(1).unwrap().as_str().split(',').collect_vec();
    let modules = modules.captures_iter(&binding).map(|c| {
        (c.get(1).unwrap().as_str().chars().next(), c.get(2).unwrap().as_str(), c.get(3).unwrap().as_str().split(", ").collect_vec())
    }).collect_vec();

    let mut modules: HashMap<&str, Module> = modules.iter().map( |(t, n, d)| {
        (*n, match t {
            Some('%') => { Module::FlipFlop { dests: d, is_on: false }}
            Some('&') => { Module::Conjunction { from: modules.iter().filter(|m| m.2.contains(&n)).map(|(_, n, _)| (*n, false)).collect_vec(), to: d }}
            None => { Module::Broadcast { dests: d }}
            Some(t) => panic!("Unrecognised char {}", t)
        })
    }).collect();

    // println!("{:?}", &modules);

    let mut pulses = VecDeque::new();
    let mut num_low: usize = 0;
    let mut num_high: usize = 0;

    for _ in 0..1000 {
        pulses.push_front(("broadcaster", false, ""));

        while let Some((dest, is_high, from)) = pulses.pop_front() {
            // println!("{} -> {} {}", from, dest, is_high);
            if is_high {
                num_high += 1;
            } else {
                num_low += 1;
            }

            if let Some(m) = modules.get_mut(dest) {
                for pulse in m.do_pulse(is_high, from, dest) {
                    pulses.push_back(pulse);
                }
            }
        }
    }

    println!("{}", num_low * num_high);

    Ok(())
}
