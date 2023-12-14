use std::{
    collections::{HashSet, HashMap},
    error::Error,
    fs, cmp::Ordering,
};

use itertools::Itertools;

enum Dir {
    N, S, E, W
}

impl Dir {
    fn min(&self, l: &(usize, usize), r: &(usize, usize)) -> Ordering {
        match self {
            Dir::N => l.1.cmp(&r.1),
            Dir::S => r.1.cmp(&l.1),
            Dir::E => r.0.cmp(&l.0),
            Dir::W => l.0.cmp(&r.0),
        }
    }

    fn increment(&self, (x, y): (usize, usize), height: usize, width: usize) -> Option<(usize, usize)> {
        match self {
            Dir::N => if y != 0 {Some((x, y-1))} else {None},
            Dir::S => if y != height - 1 {Some((x, y+1))} else {None},
            Dir::W => if x != 0 {Some((x-1, y))} else {None},
            Dir::E => if x != width - 1 {Some((x+1, y))} else {None},
        }
    }
}

fn next(map: &Vec<Vec<char>>, rounds: &Vec<(usize, usize)>, dir: Dir) -> Vec<(usize, usize)> {
    let width = map[0].len();
    let height = map.len();

    let mut new_rounds = HashSet::with_capacity(rounds.len());
    rounds.iter().sorted_by(|l, r| dir.min(l, r)).for_each(|p| {
        // println!("{x}, {y}");
        // println!("{queue:?}");
        let mut p = *p;

        loop {
            if let Some(next) = dir.increment(p, height, width) {
                if map[next.1][next.0] == '#' || new_rounds.contains(&next) {break;}
                p = next;
            } else {
                break;
            }
        }
        new_rounds.insert(p);
    });
    new_rounds.into_iter().collect_vec()
}

fn main() -> Result<(), Box<dyn Error>> {
    let binding = fs::read_to_string("inputs/day14.txt")?;

    let inputs: Vec<Vec<char>> = binding.lines().map(|l| l.chars().collect()).collect();

    let map: Vec<Vec<char>> = inputs
        .iter()
        .map(|l| {
            l.iter()
                .map(|c| match c {
                    '#' => '#',
                    _ => '.',
                })
                .collect()
        })
        .collect();

    let mut rounds: Vec<(usize, usize)> = inputs
        .iter()
        .enumerate()
        .flat_map(|(y, v)| {
            v.iter()
                .enumerate()
                .map(move |(x, c)| (x.clone(), y.clone(), c.clone()))
        })
        .filter_map(|(x, y, c)| if c == 'O' { Some((x, y)) } else { None })
        .collect();

    // rounds = next(&map, &rounds, Dir::N);
    let mut prevs: HashMap<Vec<(usize, usize)>, usize> = Default::default();
    let mut i = 0;
    while i < 1_000_000_000 {
        prevs.insert(rounds.clone(), i);
        rounds = next(&map, &rounds, Dir::N);
        rounds = next(&map, &rounds, Dir::W);
        rounds = next(&map, &rounds, Dir::S);
        rounds = next(&map, &rounds, Dir::E);
        rounds.sort();
        i += 1;
        // println!("Prevs: {prevs:?}");
        if let Some(prev) = prevs.get(&rounds) {
            println!("found loop {} to {}", prev, i);
            let diff = i - prev;
            let remaining = 1_000_000_000 - i;
            if diff != 0 {
                println!("skipped {}", (remaining / diff) * diff);
                i += (remaining / diff) * diff
            }
        }
    }
    // for _ in 0..1_000_000 {
    //     let prev = rounds;
    //     rounds= next(&map, &prev, Dir::N);
    //     rounds= next(&map, &rounds, Dir::W);
    //     rounds= next(&map, &rounds, Dir::S);
    //     rounds= next(&map, &rounds, Dir::E);

    //     if prev == rounds {
    //         println!("Reached steady state");
    //         break;
    //     }
    // }

    for y in 0..inputs.len() {
        for x in 0..inputs[0].len()  {
            print!("{}", if rounds.contains(&(x, y)) {
                'O'
            } else { map[y][x] } )
        }
        println!();
    }

    let res: usize = (0..inputs.len()).zip((1..inputs.len() + 1).rev()).map(|(y, p)| {
        (0..inputs[0].len()).filter(|&x| rounds.contains(&(x, y))).count() * (p)
    }).sum();

    println!("{res}");

    Ok(())
}
