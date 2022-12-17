use std::collections::HashMap;

use crate::utility;

pub fn run_part_05() {
    println!("Running Day 05");
    let crate_map: HashMap<usize, Vec<char>> = HashMap::new();
    if let Ok(lines) = utility::read_lines("./src/problems/prob05.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let x: Vec<char> = ip
                                    .replace('[', " ")
                                    .replace(']', " ")
                                    .chars()
                                    .collect();
                
                let char_positions: Vec<usize> = (0 ..=(x.len() - 1)/4).map(|x| 4*x + 1).collect();
                for (pos, e) in char_positions.iter().enumerate() {
                    if x[*e] != ' ' {
                    }
                } 
                
            }
        }

        println!("Part 1 ans:");
        println!("Part 2 ans:");
    }
}

