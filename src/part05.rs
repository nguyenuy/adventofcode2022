use std::collections::HashMap;
use std::fs;
use regex::Regex;

use crate::utility;

pub fn run_part_05() {
    println!("Running Day 05");
    let mut crate_stacks: Vec<Vec<char>> = Vec::new();
    let mut stacks: Vec<char> = Vec::new();

    let contents = fs::read_to_string("./src/problems/prob05.txt").unwrap();
    let problem: Vec<&str> = contents.split("\n").collect();
    let problem_index = problem.iter().position(|&x| x.trim().is_empty()).unwrap();
    
    let crates: Vec<char> = problem[problem_index-1].chars().collect();
    let mut crate_mapping: HashMap<char, Vec<char>> = HashMap::new();

    for (pos, el) in crates.iter().enumerate() {
        if *el != ' ' {
            let mut crate_stack: Vec<char> = Vec::new();
            for ind in (0 .. problem_index-1).rev() {
                let crate_row: Vec<char> = problem[ind].chars().collect();
                if crate_row[pos] != ' ' {
                    crate_stack.push(crate_row[pos]);
                } else {
                    continue;
                }
            }
            crate_mapping.insert(*el, crate_stack);
        } else {
            continue;
        }
    }

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for ind in (problem_index + 1 .. problem.len()) {
        let command = problem[ind];
        for cap in re.captures(command) {
            let amount = &cap[1];
            let src = &cap[2];
            let dest = &cap[3];
        }
        
    }

    println!("Part 1 ans:");
    println!("Part 2 ans:");
}


