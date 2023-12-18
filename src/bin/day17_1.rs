use std::{collections::HashSet, error::Error, fs, usize};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    // let binding = fs::read_to_string("inputs/test.txt")?;
    let binding = fs::read_to_string("inputs/day17.txt")?;

    let instr_regex = Regex::new(r"(\w) (\d+) \((#\w{6})\)").unwrap();

    let mut walls : HashSet<(isize, isize, char)> = Default::default();

    let mut cur: (isize, isize, char) = (0, 0, '|');
    let mut min: (isize, isize) = (0, 0);
    let mut max: (isize, isize) = (0, 0);
    for instr in instr_regex.captures_iter(&binding) {
        let dir = instr.get(1).unwrap().as_str();
        let dist = instr.get(2).unwrap().as_str().parse::<usize>().unwrap();
        // let color = instr.get(3).unwrap().as_str();

        for _ in 0..dist {
            match dir {
                "R" => cur = (cur.0 + 1, cur.1, 'R'),
                "L" => cur = (cur.0 - 1, cur.1, 'L'),
                "U" => cur = (cur.0, cur.1 - 1, 'U'),
                "D" => cur = (cur.0, cur.1 + 1, 'D'),
                _ => panic!("unrecognised dir")
            }
            min = (min.0.min(cur.0), min.1.min(cur.1));
            max = (max.0.max(cur.0), max.1.max(cur.1));
            walls.insert(cur);
        }
    }

    assert!(cur.0 == 0 && cur.1 == 0);

    for y in min.1..max.1+1 {
        for x in min.0..max.0+1 {
            if walls.contains(&(x, y, 'U')) {
                print!("U");
            } else if walls.contains(&(x, y, 'D')) {
                print!("D");
            } else if walls.contains(&(x, y, 'R')) {
                print!("R");
            } else if walls.contains(&(x, y, 'L')) {
                print!("L");
            } else {
                print!(".");
            }
        }
        println!()
    }

    println!();

    let mut num = 0;
    for y in min.1..max.1+1 {
        let mut inside = false;
        let mut x = min.0;
        while x <= max.0 {
            if walls.contains(&(x, y, 'U')) {
                print!("U");
                inside = !inside
            }

            else if walls.contains(&(x, y, 'D')) {
                print!("D");
                inside = !inside
            }

            else if walls.contains(&(x, y, 'R')) {
                let from_above = walls.contains(&(x-1, y, 'D'));

                print!("R");
                while walls.contains(&(x+1, y, 'R')) {
                    print!("R");
                    x += 1;
                }
                if walls.contains(&(x, y+1, 'D')) {
                    if !from_above {
                        inside = !inside
                    }
                } else if walls.contains(&(x, y-1, 'U')) {
                    if from_above {
                        inside = !inside
                    }
                } else {panic!("Err 1")}
            }

            else if walls.contains(&(x, y, 'L')) {
                let from_above = if walls.contains(&(x, y-1, 'U')) {
                    true
                } else if walls.contains(&(x, y+1, 'D')) {
                    false
                } else {panic!("Err 2")};
                print!("L");
                x += 1;

                while walls.contains(&(x, y, 'L')) {
                    print!("L");
                    x += 1;
                }

                if walls.contains(&(x, y, 'U')) {
                    print!("U");
                    if from_above {
                        inside = !inside
                    }
                } else if walls.contains(&(x, y, 'D')) {
                    print!("D");
                    if !from_above {
                        inside = !inside
                    }
                } else {
                    panic!("Err 3")
                }
            } else {
                if inside {
                    num+=1;
                    print!("#");
                } else {
                    print!(".");
                }
            }
            x += 1;
        }
        println!();
    }

    let res = num + walls.len();
    println!("{res}");

    Ok(())
}
