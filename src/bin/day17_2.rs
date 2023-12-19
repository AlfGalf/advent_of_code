use std::{error::Error, fs, usize};

use itertools::Itertools;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    // let binding = fs::read_to_string("inputs/test.txt")?;
    let binding = fs::read_to_string("inputs/day17.txt")?;

    let instr_regex = Regex::new(r"(\w) (\d+) \(#(\w{6})\)").unwrap();

    let mut walls : Vec<((isize, isize), (isize, isize), char)> = Default::default();

    let mut cur: (isize, isize);
    let mut next: (isize, isize) = (0, 0);
    let mut min: (isize, isize) = (0, 0);
    let mut max: (isize, isize) = (0, 0);
    for instr in instr_regex.captures_iter(&binding) {

        // let dir = instr.get(1).unwrap().as_str().chars().next().unwrap();
        // let dist = instr.get(2).unwrap().as_str().parse::<isize>().unwrap();

        let color = instr.get(3).unwrap().as_str();
        // println!("{}", color);
        let dir = match &color[5..6] {
            "0" => 'R',
            "1" => 'D',
            "2" => 'L',
            "3" => 'U',
            _ => panic!("Err!")
        };
        let dist = isize::from_str_radix(&color[0..5], 16).unwrap();
        println!("{} {}", dir, dist);

        cur = next;
        match dir {
            'R' => next = (cur.0 + dist, cur.1),
            'L' => next = (cur.0 - dist, cur.1),
            'U' => next = (cur.0, cur.1 - dist),
            'D' => next = (cur.0, cur.1 + dist),
            _ => panic!("unrecognised dir")
        }
        min = (min.0.min(cur.0), min.1.min(cur.1));
        max = (max.0.max(cur.0), max.1.max(cur.1));
        walls.push((cur, next, dir));
    }

    assert!(next.0 == 0 && next.1 == 0);

    let mut num: usize = 0;

    let count_row = |y: isize| {
        let mut num = 0;
        let walls_y = walls.iter().filter(|&&w| match w {
            ((_sx, sy), (_ex, ey), 'U') => sy > y && ey < y,
            ((_sx, sy), (_ex, ey), 'D') => sy < y && ey > y,
            ((_sx, sy), (_ex, _ey), 'R') => y == sy,
            ((_sx, sy), (_ex, _ey), 'L') => y == sy,
            _ => false
        });

        let mut walls_y = walls_y.collect_vec();

        walls_y.sort_by(|l, r| l.0.0.cmp(&r.0.0));

        let mut inside = false;
        let mut last_x = isize::MIN;
        for &((sx, sy), (dx, dy), d) in walls_y {
            let prev_wall = walls.iter().filter(|&&(_,e, _)| e == (sx, sy)).next().unwrap();
            let next_wall = walls.iter().filter(|&&(s,_, _)| s == (dx, dy)).next().unwrap();
            // Increase by internal number
            if inside { num += last_x.abs_diff(sx.min(dx))};

            match d {
                'U' => {
                    inside = !inside;
                    last_x = sx+1;
                    // print!("#");
                    num+=1
                }
                'D' => {
                    inside = !inside;
                    last_x = sx+1;
                    // print!("#");
                    num+=1
                }
                'L' => {
                    // println!("{} {}", prev_wall.2, next_wall.2);
                    if prev_wall.2 == next_wall.2 {inside = !inside;}
                    last_x = sx+1;
                    // for x in dx..sx+1 {print!("-");}
                    num += sx.abs_diff(dx)+1;
                 }
                'R' => {
                    if prev_wall.2 == next_wall.2 {inside = !inside;}
                    last_x = dx+1;
                    // for x in sx..dx+1 {print!("-");}
                    num += sx.abs_diff(dx)+1;
                }
                _ => panic!()
            }
        }
        num
    };

    let mut last_count = 0;
    let mut last_y = isize::MIN;
    for y in walls.iter().filter(|&&(_,_,c)| c == 'L' || c == 'R').map(|((_,y),_,_)| y).unique().sorted() {
        println!("{}", y);
        num += count_row(*y);
        num += last_count * (y.abs_diff(last_y) - 1);
        last_count = count_row(*y+1);
        last_y = *y;
    }

    println!("{num}");

    Ok(())
}
