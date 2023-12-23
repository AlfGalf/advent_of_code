use std::{collections::HashSet, error::Error, fs, usize};

use itertools::Itertools;
use regex::Regex;

type BLOCK = ([usize; 3], [usize; 3]);

fn max_point(l: &[usize; 3], r: &[usize; 3]) -> [usize; 3] {
    [l[0].max(r[0]), l[1].max(r[1]), l[2].max(r[2])]
}

fn min_point(l: &[usize; 3], r: &[usize; 3]) -> [usize; 3] {
    [l[0].min(r[0]), l[1].min(r[1]), l[2].min(r[2])]
}

fn main() -> Result<(), Box<dyn Error>> {
    // let binding = fs::read_to_string("inputs/test.txt")?;
    let binding = fs::read_to_string("inputs/day22.txt")?;

    let blocks = Regex::new(r"(?m)^(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)").unwrap();
    let mut blocks: Vec<BLOCK> = blocks
        .captures_iter(&binding)
        .map(|c| {
            (
                [
                    c.get(1).unwrap().as_str().parse().unwrap(),
                    c.get(2).unwrap().as_str().parse().unwrap(),
                    c.get(3).unwrap().as_str().parse().unwrap(),
                ],
                [
                    c.get(4).unwrap().as_str().parse().unwrap(),
                    c.get(5).unwrap().as_str().parse().unwrap(),
                    c.get(6).unwrap().as_str().parse().unwrap(),
                ],
            )
        })
        .collect_vec();

    blocks.sort_by(|([_, _, l1z], [_, _, r1z]), ([_, _, l2z], [_, _, r2z])| {
        (l1z.min(r1z)).cmp(l2z.min(r2z))
    });

    let max = blocks
        .iter()
        .fold([0, 0, 0], |p, (l, r)| max_point(&p, &max_point(l, r)));

    println!("{:?}", max);

    let mut map: Vec<Vec<Vec<Option<usize>>>> = (0..=max[0])
        .into_iter()
        .map(|_| {
            (0..=max[1])
                .into_iter()
                .map(|_| (0..=max[2]).into_iter().map(|_| None).collect_vec())
                .collect_vec()
        })
        .collect_vec();

    let mut new_blocks: Vec<(BLOCK, HashSet<usize>, HashSet<usize>)> = Default::default();

    // let blocks = blocks.into_iter().map
    for (i, mut block) in blocks.into_iter().enumerate() {
        println!("{:?}", block);
        let mut rely_on = HashSet::new();
        'inner: while { block.0[2].min(block.1[2]) > 1 } {
            let low = min_point(&block.0, &block.1);
            let high = max_point(&block.0, &block.1);

            let mut does_break = false;
            for x in low[0]..=high[0] {
                for y in low[1]..=high[1] {
                    // println!("x {} y {} z {}", x, y, low[2]-1);
                    if let Some(b) = map[x][y][low[2] - 1] {
                        new_blocks[b].1.insert(i);
                        rely_on.insert(b);
                        does_break = true;
                    }
                }
            }
            if does_break {
                break 'inner;
            }

            block.0[2] -= 1;
            block.1[2] -= 1;
        }

        let low = min_point(&block.0, &block.1);
        let high = max_point(&block.0, &block.1);

        for x in low[0]..=high[0] {
            for y in low[1]..=high[1] {
                for z in low[2]..=high[2] {
                    // println!("set x {} y {} z {}", x, y, low[2]-1);
                    map[x][y][z] = Some(i)
                }
            }
        }

        new_blocks.push((block, HashSet::new(), rely_on));
    }

    println!("{:?}", new_blocks);
    println!(
        "{:?}",
        new_blocks
            .iter()
            .filter(|(_, v, _)| v.iter().all(|bi| new_blocks[*bi].2.len() > 1))
            .count()
    );

    Ok(())
}
