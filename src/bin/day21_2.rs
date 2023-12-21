use std::{error::Error, fs, usize, collections::HashSet};

use itertools::Itertools;

const DIRS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const LEN: usize = 26501365;

fn main() -> Result<(), Box<dyn Error>> {
    // let binding = fs::read_to_string("inputs/test.txt")?;
    let binding = fs::read_to_string("inputs/day21.txt")?;

    let map = binding.lines().map(|l| l.chars().map(|c| if c == 'S' {'.'} else {c}).collect_vec()).collect_vec();

    let width = map[0].len();
    let height = map.len();

    let start: (isize, isize) = binding
        .lines()
        .enumerate()
        .flat_map(|(y, v)| {
            v.chars()
                .enumerate()
                .map(move |(x, c)| (x.clone(), y.clone(), c.clone()))
        })
        .filter_map(|(x, y, c)| if c == 'S' { Some((x as isize, y as isize)) } else { None })
        .next().unwrap();

    assert!(width == height);
    println!("{}", width);

    let get_res = |n| {
        let mut arr = HashSet::new();
        arr.insert(start);
        for _ in 0..n {
            let mut new_arr = HashSet::new();
            for (x, y) in arr {
                for (xd, yd) in DIRS {
                    let (px, py) = (x as isize + xd, y as isize + yd);

                    if map[(py.rem_euclid(height as isize)) as usize][(px.rem_euclid(width as isize)) as usize] == '.' {
                        new_arr.insert((px, py));
                    }
                }
            }
            arr = new_arr
        }
        arr.len()
    };

    for i in 0..5 {
        println!("{}: {}", width * i + 65, get_res(width * i + 65))
        // Solve quadratic manually
    }

    Ok(())
}
