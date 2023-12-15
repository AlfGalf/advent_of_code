use std::{error::Error, fs};

fn check(guess: &str, nums: &Vec<usize>) -> Option<bool> {
    let total = nums.iter().sum();
    let cur_yes = guess.chars().filter(|c| c == &'#').count();
    let cur_q = guess.chars().filter(|c| c == &'?').count();

    if cur_yes > total || cur_q + cur_yes < total {
        return Some(false);
    }

    if guess.contains('?') {
        if let Some(answered_section) = guess.split('?').next() {
            for (num, part) in answered_section
                .split('.')
                .filter(|&x| !x.is_empty())
                .collect::<Vec<&str>>()
                .into_iter()
                .enumerate()
                .rev()
                .skip(1)
            {
                let Some(&n) = nums.get(num) else {
                    return Some(false);
                };
                if part.len() != n {
                    return Some(false);
                }
            }
            None
        } else {
            // No current guesses
            None
        }
    } else {
        for (num, part) in guess.split('.').filter(|&x| !x.is_empty()).enumerate() {
            let Some(&n) = nums.get(num) else {
                return Some(false);
            };
            if part.len() != n {
                return Some(false);
            }
        }
        Some(true)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let binding = fs::read_to_string("inputs/day12.txt")?;

    let res: usize = binding
        .lines()
        .map(|l| {
            let (chars, nums) = l.split_once(" ").expect(&format!("Brken string input {l}"));

            let nums: Vec<usize> = nums.split(',').map(|s| s.parse().unwrap()).collect();

            let mut num_correct = 0;
            let mut guesses: Vec<String> = Default::default();

            guesses.push(chars.to_string());

            while let Some(guess) = guesses.pop() {
                match check(&guess, &nums) {
                    Some(true) => num_correct += 1,
                    Some(false) => {}
                    None => {
                        let index = guess
                            .find('?')
                            .expect("There should definitely be a ? if check is not certain");
                        let mut guess_1 = guess.to_string();
                        guess_1.replace_range(index..index + 1, "#");
                        let mut guess_2 = guess.to_string();
                        guess_2.replace_range(index..index + 1, ".");
                        guesses.push(guess_1);
                        guesses.push(guess_2);
                    }
                }
            }
            num_correct
        })
        .sum();

    println!("{res}");

    Ok(())
}
