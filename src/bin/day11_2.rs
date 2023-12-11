use std::{fs, error::Error};
use itertools::iproduct;

fn main() -> Result<(), Box<dyn Error>> {
    let binding = fs::read_to_string("inputs/day11.txt")?;

    let chars: Vec<Vec<char>> = binding.lines().map(|l| l.chars().collect()).collect();

    let empty_rows: Vec<bool> = chars.iter().map(|v| v.iter().all(|c| c == &'.')).collect();
    let empty_cols: Vec<bool> = (0..chars[0].len()).map(|r| chars.iter().map(|v| v[r]).all(|c| c == '.')).collect();

    let pos: Vec<(usize, usize)> = chars.iter().enumerate().flat_map(|(y, v)| v.iter().enumerate().map(move |(x, c)| (x.clone(), y.clone(), c.clone()))).filter_map(|(x, y, c)| if c == '#' {Some((x, y))} else {None}).collect();

    println!("{pos:?} {empty_rows:?}, {empty_cols:?}");

    // let multiplier = 10 - 1;
    // let multiplier = 100 - 1;
    let multiplier = 1_000_000 - 1;
    let prod: usize = iproduct!(&pos, &pos).map(|((fx, fy),(tx, ty))| {
        print!("({fx}, {fy}) ({tx}, {ty}), ");
        let (fx, tx) = if fx < tx {(*fx, *tx)} else {(*tx, *fx)};
        let (fy, ty) = if fy < ty {(*fy, *ty)} else {(*ty, *fy)};

        let num_rows = empty_rows[fy..ty].iter().filter(|b| **b).count();
        let num_cols = empty_cols[fx..tx].iter().filter(|b| **b).count();

        let res = fx.abs_diff(tx) + fy.abs_diff(ty) + num_cols * multiplier + num_rows * multiplier;
        println!("{}", res);
        res
        // fx.abs_diff(tx) + fy.abs_diff(ty) + num_cols * 100 + num_rows * 100
    }).sum::<usize>() / 2;

    println!("{prod}");

    Ok(())
}