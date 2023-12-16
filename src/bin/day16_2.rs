use std::{collections::VecDeque, error::Error, fs, usize};

use itertools::Itertools;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    fn advance(
        &self,
        (x, y): (usize, usize),
        width: usize,
        height: usize,
    ) -> Option<(usize, usize)> {
        match self {
            Dir::N => {
                if y > 0 {
                    Some((x, y - 1))
                } else {
                    None
                }
            }
            Dir::S => {
                if y < height - 1 {
                    Some((x, y + 1))
                } else {
                    None
                }
            }
            Dir::E => {
                if x < width - 1 {
                    Some((x + 1, y))
                } else {
                    None
                }
            }
            Dir::W => {
                if x > 0 {
                    Some((x - 1, y))
                } else {
                    None
                }
            }
        }
    }
}

fn calc(map: &Vec<Vec<char>>, (x, y): (usize, usize), dir: Dir) -> usize {
    let width = map[0].len();
    let height = map.len();

    let mut energised: Vec<Vec<Vec<Dir>>> = map
        .iter()
        .map(|l| l.iter().map(|_| vec![]).collect())
        .collect();

    let mut frontier: VecDeque<(usize, usize, Dir)> = Default::default();
    frontier.push_front((x, y, dir));

    while let Some((x, y, dir)) = frontier.pop_front() {
        if energised[y][x].contains(&dir) {
            continue;
        }
        energised[y][x].push(dir);

        let mut add_to = |p: Option<(usize, usize)>, dir: Dir| {
            if let Some((x, y)) = p {
                frontier.push_back((x, y, dir));
            }
        };

        match (map[y][x], dir) {
            ('.', dir) => add_to(dir.advance((x, y), width, height), dir),
            ('|', dir) if dir == Dir::N || dir == Dir::S => {
                add_to(dir.advance((x, y), width, height), dir)
            }
            ('|', dir) if dir == Dir::E || dir == Dir::W => {
                add_to(Dir::N.advance((x, y), width, height), Dir::N);
                add_to(Dir::S.advance((x, y), width, height), Dir::S);
            }
            ('-', dir) if dir == Dir::E || dir == Dir::W => {
                add_to(dir.advance((x, y), width, height), dir)
            }
            ('-', dir) if dir == Dir::N || dir == Dir::S => {
                add_to(Dir::E.advance((x, y), width, height), Dir::E);
                add_to(Dir::W.advance((x, y), width, height), Dir::W);
            }
            ('/', dir) => match dir {
                Dir::N => add_to(Dir::E.advance((x, y), width, height), Dir::E),
                Dir::S => add_to(Dir::W.advance((x, y), width, height), Dir::W),
                Dir::E => add_to(Dir::N.advance((x, y), width, height), Dir::N),
                Dir::W => add_to(Dir::S.advance((x, y), width, height), Dir::S),
            },
            ('\\', dir) => match dir {
                Dir::N => add_to(Dir::W.advance((x, y), width, height), Dir::W),
                Dir::S => add_to(Dir::E.advance((x, y), width, height), Dir::E),
                Dir::E => add_to(Dir::S.advance((x, y), width, height), Dir::S),
                Dir::W => add_to(Dir::N.advance((x, y), width, height), Dir::N),
            },
            _ => panic!("Unrecognised char {}", map[y][x]),
        }
    }

    for y in 0..height {
        for x in 0..width {
            print!("{}", if energised[y][x].is_empty() { '.' } else { '#' });
        }
        println!();
    }

    let res = energised
        .iter()
        .flat_map(|l| l.iter().filter(|v| !v.is_empty()))
        .count();
    println!("{}", res);
    res
}

fn main() -> Result<(), Box<dyn Error>> {
    let binding = fs::read_to_string("inputs/day16.txt")?;

    let map: Vec<Vec<char>> = binding.lines().map(|l| l.chars().collect()).collect();

    let mut max = 0;
    for x in 0..map[0].len() {
        max = max.max(calc(&map, (x, 0), Dir::S));
        max = max.max(calc(&map, (x, map.len()-1), Dir::N));
    }
    for y in 0..map.len() {
        max = max.max(calc(&map, (0, y), Dir::E));
        max = max.max(calc(&map, (map[0].len()-1, y), Dir::W));
    }

    println!("{max}");

    Ok(())
}
