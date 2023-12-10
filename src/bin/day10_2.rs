use std::{fs, error::Error, collections::{VecDeque, HashSet}};


fn main() -> Result<(), Box<dyn Error>> {
    let binding = fs::read_to_string("inputs/day10.txt")?;

    let pipes: Vec<Vec<char>> = binding.lines().map(|l| l.chars().collect()).collect();

    let mut walls: HashSet<(usize, usize)> = Default::default();

    let start_pos = pipes.iter().enumerate().flat_map(|(y, v)| v.iter().enumerate().map(move |(x, c)| (x.clone(), y.clone(), c.clone()))).filter(|(_, _, c)| *c == 'S').next().unwrap();
    let start_pos = (start_pos.0, start_pos.1);
    // let animalPos = pipes.iter().map(|l| l.iter().enumerate()).flatten().enumerate().filter(|(_, (_, c))| **c == 'A').map(|(x, (y, _))| (x,y)).next().unwrap();

    let mut frontier = VecDeque::new();
    frontier.push_front(start_pos);
    walls.insert(start_pos);

    let connected = |from_char: char, from_pos: (usize, usize), next_pos: (usize, usize)| -> bool {
        match (from_char, from_pos, next_pos) {
            ('|', (fx, fy), (nx, ny)) if (nx, ny) == (fx, fy + 1) || (nx, ny) == (fx, fy - 1) => { true }
            ('-', (fx, fy), (nx, ny)) if (nx, ny) == (fx + 1, fy) || (nx, ny) == (fx - 1, fy) => { true }
            ('7', (fx, fy), (nx, ny)) if (nx, ny) == (fx - 1, fy) || (nx, ny) == (fx, fy + 1) => { true }
            ('J', (fx, fy), (nx, ny)) if (nx, ny) == (fx - 1, fy) || (nx, ny) == (fx, fy - 1) => { true }
            ('L', (fx, fy), (nx, ny)) if (nx, ny) == (fx + 1, fy) || (nx, ny) == (fx, fy - 1) => { true }
            ('F', (fx, fy), (nx, ny)) if (nx, ny) == (fx + 1, fy) || (nx, ny) == (fx, fy + 1) => { true }
            ('S', _, _) => { true }
            ('A', _, _) => { true }
            _ => false
        }
    };

    let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    while let Some(pos) = frontier.pop_front() {
        // println!("Exploring {pos:?}");
        let Some(from_pipe) = pipes.get(pos.1).map(|v| v.get(pos.0)).flatten() else {continue;};
        for dir in &dirs {
            let next_pos = ((pos.0 as isize + dir.0) as usize, (pos.1 as isize + dir.1) as usize);
            // println!("{next_pos:?}");
            let Some(next_pipe) = pipes.get(next_pos.1).map(|v| v.get(next_pos.0)).flatten() else {continue;};
            // println!("{next_pipe:?}");
            if connected(*from_pipe, pos, next_pos) && connected(*next_pipe, next_pos, pos) {
                if !walls.contains(&next_pos) {
                    frontier.push_back(next_pos);
                    walls.insert(next_pos);
                }
            }
        }
    }

    let mut num = 0;
    for y in 0..pipes.len() {
        let mut inside = false;
        let mut from_above = false;
        for x in 0..pipes[0].len() {
            let pipe = pipes[y][x];

            if walls.contains(&(x, y)) {
                print!("{pipe}");
                if pipe == '|' { inside = !inside }
                if pipe == 'F' { from_above = false }
                if pipe == 'L' { from_above = true }
                if pipe == 'J' && !from_above { inside = !inside }
                if (pipe == '7' || pipe == 'S') && from_above { inside = !inside }

                // if inside % 2 == 1 {
                //     if pipe == '|' || pipe == 'J' || pipe == '7' || pipe == 'S' { inside -= 1 }
                // } else {
                //     if pipe == '|' || pipe == 'F' || pipe == 'L' { inside += 1; };
                // }
            } else if inside {
                num += 1;
                print!("I");
            } else {
                print!("O");
            }
        }
        println!("");
    }

    // println!("{dist:?}");
    println!("{}", num);

    Ok(())
}