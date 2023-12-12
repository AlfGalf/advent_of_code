use std::{fs, error::Error, collections::{VecDeque, HashMap, HashSet}, usize};

// Advances to the next ?
fn advance(chars: &[char], nums: &[usize], cur: (usize, usize)) -> Option<(usize, usize)> {
    let (mut cur_c_i, mut cur_n_i) = cur;

    // println!("adv {} {}", cur_c_i, cur_n_i);
    while cur_c_i < chars.len() && chars[cur_c_i] != '?' {
        // println!("cur {} {}", cur_c_i, cur_n_i);
        if chars[cur_c_i] == '.' {
            cur_c_i += 1
        }
        else {
            assert!(chars[cur_c_i] == '#');
            // If can fit the current bunch in
            // println!("{:?} {}", &chars[cur_c_i..(cur_c_i+nums[cur_n_i])], if cur_c_i + nums[cur_n_i] < chars.len() {chars[cur_c_i + nums[cur_n_i]]} else {'E'});
            if cur_n_i < nums.len() && cur_c_i + nums[cur_n_i] <= chars.len() && chars[cur_c_i..cur_c_i+nums[cur_n_i]].iter().all(|&c| c != '.') && (cur_c_i + nums[cur_n_i] >= chars.len() || chars[cur_c_i + nums[cur_n_i]] != '#') {
                cur_c_i += nums[cur_n_i] + 1;
                cur_c_i = cur_c_i.min(chars.len());
                cur_n_i += 1;
            } else {
                // println!("didnt fit");
                return None;
            }
        }
    }
    // println!("res {} {}", cur_c_i, cur_n_i);
    Some((cur_c_i, cur_n_i))
}

fn main() -> Result<(), Box<dyn Error>> {
    let binding = fs::read_to_string("inputs/day12.txt")?;

    let res: usize = binding.lines().map(|l| {
        let (chars, nums) = l.split_once(" ").expect(&format!("Brken string input {l}"));
        let nums: Vec<usize> = nums.split(',').map(|s| s.parse().unwrap()).collect();

        let mut chars = chars.to_string();
        chars.push('?');
        let chars = chars.repeat(5);
        let chars: Vec<char> = chars[0..chars.len() - 1].to_string().chars().collect();
        // let chars: Vec<char> = chars.chars().collect();
        // println!("{:?}", chars);
        // println!("{:?}", nums);

        let nums = nums.repeat(5);


        // println!("{chars} {nums:?}");

        // let mut num_correct = 0;
        // Guesses = (usize, usize), num chars used, num solved
        // hashmap of num ways to get to a point with that num solved
        let mut guesses: HashSet<(usize, usize)> = Default::default();
        let mut ways: HashMap<(usize, usize), usize> = Default::default();

        if let Some(start) = advance(&chars, &nums, (0, 0)) {
             ways.insert(start, 1);
             guesses.insert(start);

             while let Some(&(g_ind, n_ind)) = guesses.iter().min_by(|(c1, _), (c2, _)| c1.cmp(c2)) {
                // if g_ind >= chars.len() {break;}
                guesses.remove(&(g_ind, n_ind));

                if g_ind == chars.len() {continue;}

                let cur_ways = *ways.get(&(g_ind, n_ind)).unwrap();
                // println!("Exploring ({}, {}): {}", g_ind, n_ind, cur_ways);

                // try with both # and .
                let mut guess = chars.clone();
                guess[g_ind] = '.';
                // println!("{:?}", guess);
                if let Some(ind) = advance(&guess, &nums, (g_ind, n_ind)) {
                    *ways.entry(ind).or_insert(0) += cur_ways;
                    guesses.insert(ind);
                    // println!("Added {ind:?}");
                }

                guess[g_ind] = '#';
                // println!("{:?}", guess);
                if let Some(ind) = advance(&guess, &nums, (g_ind, n_ind)) {
                    *ways.entry(ind).or_insert(0) += cur_ways;
                    guesses.insert(ind);
                    // println!("Added {ind:?}");
                }
            }
        }

        let ans = *ways.get(&(chars.len(), nums.len())).unwrap_or(&0);
        println!("ANS: {}", ans);
        ans
    }).sum();

    println!("FINAL: {res}");

    Ok(())
}