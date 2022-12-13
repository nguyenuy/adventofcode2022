use crate::utility;

pub fn run_part_03a() {
    println!("Running Day 03 - Part A");
    let mut rucksack: Vec<String> = Vec::new();

    // Build the problem vectors
    if let Ok(lines) = utility::read_lines("./src/problems/prob02a.txt") {
        for line in lines {
            if let Ok(ip) = line {
               rucksack.push(ip); 
            }
        }
    }

    println!("Part 1 ans:");
}
