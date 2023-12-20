use std::{error::Error, fs, usize, collections::{HashMap, VecDeque}};

use regex::Regex;

type Part = [(usize, usize); 4];

#[derive(Clone)]
enum Dest<'a> {
    Accept,
    Reject,
    Workflow(&'a str)
}

impl<'a> Dest<'a> {
    fn from_str(s: &'a str) -> Self {
        if s == "A" {
            Self::Accept
        } else if s == "R" {
            Self::Reject
        } else {
            Self::Workflow(s)
        }
    }
}

enum Rule<'a> {
    GT{ind: usize, req: usize, dst: Dest<'a>},
    LT{ind: usize, req: usize, dst: Dest<'a>},
}

impl Rule<'_> {
    fn apply_rule(&self, ranges: Part) -> ((&Dest, Part), Part) {
        match self {
            Rule::GT { ind, req, dst } => {
                let range = &ranges[*ind];
                let mut good_range = ranges.clone();
                let mut bad_range = ranges.clone();
                good_range[*ind] = (range.0.max(*req+1), range.1);
                bad_range[*ind] = (range.0, range.1.min(*req));
                ((dst, good_range), bad_range)
            },
            Rule::LT { ind, req, dst } => {
                let range = &ranges[*ind];
                let mut good_range = ranges.clone();
                let mut bad_range = ranges.clone();
                good_range[*ind] = (range.0,range.1.min(*req-1));
                bad_range[*ind] = (range.0.max(*req),range.1);

                ((dst, good_range), bad_range)
            },
        }
    }
}

struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    default: Dest<'a>
}

impl<'a> Workflow<'a> {
    fn from_str(s: &'a str) -> Self {
        let lt_rule = Regex::new(r"^(\w)<(\d+):(\w+)$").unwrap();
        let gt_rule = Regex::new(r"^(\w)>(\d+):(\w+)$").unwrap();

        let mut rules = s.split(',').rev();
        let last = rules.next().unwrap();
        Self {
            rules: rules.rev().map(|s| {
                if let Some(c) = lt_rule.captures(s) {
                    Rule::LT { ind: str_to_ind(c.get(1).unwrap().as_str()), req: c.get(2).unwrap().as_str().parse().unwrap(), dst: Dest::from_str(c.get(3).unwrap().as_str()) }
                } else if let Some(c) = gt_rule.captures(s) {
                    Rule::GT { ind: str_to_ind(c.get(1).unwrap().as_str()), req: c.get(2).unwrap().as_str().parse().unwrap(), dst: Dest::from_str(c.get(3).unwrap().as_str()) }
                } else {
                    panic!("Didnt recognise rule {s}")
                }
            }).collect(),
            default: Dest::from_str(last)
        }
    }
}


fn str_to_ind(s: &str) -> usize {
    match s {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!("unrecognised part symbos {s}")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // let binding = fs::read_to_string("inputs/test.txt")?;
    let binding = fs::read_to_string("inputs/day18.txt")?;

    let workflow = Regex::new(r"(?m)^(.+)\{(.+)\}$")?;

    let workflows: HashMap<&str, Workflow> = workflow.captures_iter(&binding).map(|c| {
        let name: &str = c.get(1).unwrap().as_str();
        let rules: Workflow = Workflow::from_str(c.get(2).unwrap().as_str());
        (name, rules)
    }).collect();

    let mut stack: VecDeque<(&Dest, Part)> = VecDeque::new();
    let start_range = (&Dest::Workflow("in"), [(1, 4000), (1, 4000), (1, 4000), (1, 4000)]);
    stack.push_front(start_range);

    let mut res: usize = 0;
    while let Some((cur_state, mut p)) = stack.pop_front() {
        match cur_state {
            Dest::Accept => {
                res += (p[0].1 - p[0].0 + 1) * (p[1].1 - p[1].0 + 1) * (p[2].1 - p[2].0 + 1) * (p[3].1 - p[3].0 + 1)
            }
            Dest::Reject => {}
            Dest::Workflow(str) => {
                let workflow = &workflows[str];

                for rule in &workflow.rules {
                    let (y, n) = rule.apply_rule(p);
                    stack.push_back(y);
                    p = n;
                }
                stack.push_back((&workflow.default, p));
            }
        }
    }

    println!("{res}");

    Ok(())
}
