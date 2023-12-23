use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fs, usize,
};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    // let binding = fs::read_to_string("inputs/test.txt")?;
    let binding = fs::read_to_string("inputs/day23.txt")?;

    let map = binding
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let height = map.len();
    let width = map[0].len();

    let start = (
        map[0]
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == '.')
            .next()
            .unwrap()
            .0,
        0,
    );
    let end = (
        map[height - 1]
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == '.')
            .next()
            .unwrap()
            .0,
        height - 1,
    );

    let mut nodes: Vec<(usize, usize)> = (1..width - 1)
        .flat_map(|x| (1..height - 1).map(move |y| (x, y)))
        .filter_map(|(x, y)| {
            if map[y][x] == '#' {
                return None;
            }
            let mut dirs = vec![];
            if map[y - 1][x] != '#' {
                dirs.push((x, y - 1));
            }
            if map[y + 1][x] != '#' {
                dirs.push((x, y + 1));
            }
            if map[y][x - 1] != '#' {
                dirs.push((x - 1, y));
            }
            if map[y][x + 1] != '#' {
                dirs.push((x + 1, y));
            }
            if dirs.len() > 2 {
                Some((x, y))
            } else {
                None
            }
        })
        .collect_vec();

    nodes.push(start);
    nodes.push(end);

    println!("{:?}", nodes);

    let mut connections: HashMap<usize, Vec<(usize, usize)>> = Default::default();

    for node_i in 0..nodes.len() {
        let explore = |mut cur: (usize, usize), mut prev: (usize, usize)| {
            let mut possible_nodes = vec![];
            if map[cur.1][cur.0] == '#' {
                return None;
            }
            let mut len = 1;
            loop {
                if let Some(n) = nodes.iter().position(|&n| n == cur) {
                    return Some((n, len));
                }

                if cur.1 > 0 {
                    let this = (cur.0, cur.1 - 1);
                    if prev != this && (map[cur.1 - 1][cur.0] != '#') {
                        possible_nodes.push(this);
                    }
                }
                if cur.1 + 1 < height {
                    let this = (cur.0, cur.1 + 1);
                    if prev != this && (map[cur.1 + 1][cur.0] != '#') {
                        possible_nodes.push(this);
                    }
                }
                if cur.0 > 0 {
                    let this = (cur.0 - 1, cur.1);
                    if prev != this && (map[cur.1][cur.0 - 1] != '#') {
                        possible_nodes.push(this);
                    }
                }
                if cur.0 + 1 < width {
                    let this = (cur.0 + 1, cur.1);
                    if prev != this && (map[cur.1][cur.0 + 1] != '#') {
                        possible_nodes.push(this);
                    }
                }
                if possible_nodes.is_empty() {
                    break None;
                } else if possible_nodes.len() == 1 {
                    prev = cur;
                    cur = possible_nodes.pop().unwrap();
                    len += 1;
                    continue;
                } else {
                    panic!("Should not have happened")
                }
            }
        };

        let node = nodes[node_i];
        connections.insert(node_i, Default::default());
        if node.1 > 0 {
            if let Some(up) = explore((node.0, node.1 - 1), node) {
                connections.get_mut(&node_i).unwrap().push(up);
            }
        }
        if node.1 + 1 < height {
            if let Some(down) = explore((node.0, node.1 + 1), node) {
                connections.get_mut(&node_i).unwrap().push(down);
            }
        }
        if node.0 > 0 {
            if let Some(left) = explore((node.0 - 1, node.1), node) {
                connections.get_mut(&node_i).unwrap().push(left);
            }
        }
        if node.0 + 1 < width {
            if let Some(right) = explore((node.0 + 1, node.1), node) {
                connections.get_mut(&node_i).unwrap().push(right);
            }
        }
    }

    println!("{:?}", connections);

    let mut frontier: VecDeque<(usize, HashSet<usize>, usize)> = Default::default();

    let start = nodes.iter().position(|&n| n == start).unwrap();
    let end = nodes.iter().position(|&n| n == end).unwrap();

    frontier.push_front((start, Default::default(), 0));

    let mut best = 0;
    while let Some((i, past, cur_len)) = frontier.pop_front() {
        if i == end {
            best = best.max(cur_len);
            println!("End with len {}, max {}", cur_len, best);
            continue;
        };

        for &(to, len) in &connections[&i] {
            if !past.contains(&to) {
                let mut past = past.clone();
                past.insert(i);
                frontier.push_front((to, past, cur_len + len))
            }
        }
    }

    println!("{}", best);

    Ok(())
}
