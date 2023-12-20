use std::{error::Error, fs, usize, collections::HashMap};

use itertools::Itertools;
use regex::Regex;

type Part = [usize; 4];

#[derive(Clone)]
enum Dest {
    Accept,
    Reject,
    Workflow(String)
}

impl Dest {
    fn from_str(s: &str) -> Self {
        if s == "A" {
            Self::Accept
        } else if s == "R" {
            Self::Reject
        } else {
            Self::Workflow(s.to_string())
        }
    }
}

enum Rule {
    GT{ind: usize, req: usize, dst: Dest},
    LT{ind: usize, req: usize, dst: Dest},
}

impl Rule {
    fn is_match(&self, part: &Part) -> Option<Dest> {
        match self {
            Rule::GT { ind, req, dst } => if part[*ind] > *req {Some(dst.clone())} else {None}
            Rule::LT { ind, req, dst } => if part[*ind] < *req {Some(dst.clone())} else {None},
        }
    }
}

struct Workflow {
    rules: Vec<Rule>,
    default: Dest
}

impl Workflow {
    fn from_str(s: &str) -> Self {
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

    fn get_dest(&self, part: &Part) -> Dest {
        for rule in &self.rules {
            if let Some(dest) = &rule.is_match(&part) {
                return dest.clone();
            }
        }
        return self.default.clone();
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
    let part = Regex::new(r"(?m)^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$")?;

    let workflows: HashMap<String, Workflow> = workflow.captures_iter(&binding).map(|c| {
        let name: String = c.get(1).unwrap().as_str().to_string();
        let rules: Workflow = Workflow::from_str(c.get(2).unwrap().as_str());
        (name, rules)
    }).collect();

    fn is_acc(map: &HashMap<String, Workflow>, part: &Part) -> bool {
        let mut res = map["in"].get_dest(part);
        while let Dest::Workflow(s) = res {
            res = map[&s].get_dest(part);
        }
        if let Dest::Accept = res {true} else {false}
    }

    let parts: Vec<Part> = part.captures_iter(&binding).map(|c| {
        println!("{}", c.get(0).unwrap().as_str());
        [
            c.get(1).unwrap().as_str().parse().unwrap(),
            c.get(2).unwrap().as_str().parse().unwrap(),
            c.get(3).unwrap().as_str().parse().unwrap(),
            c.get(4).unwrap().as_str().parse().unwrap(),
        ]
    }).collect_vec();

    let res: usize = parts.into_iter().filter(|p| is_acc(&workflows, p)).map(|[a, b, c, d]| a + b + c + d).sum();

    println!("{res}");

    Ok(())
}
