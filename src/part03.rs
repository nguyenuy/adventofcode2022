use std::collections::HashSet;

use crate::utility;

pub fn run_part_03a() {
    println!("Running Day 03 - Part A");
    let mut rucksack: Vec<String> = Vec::new();

    // Build the problem vectors
    if let Ok(lines) = utility::read_lines("./src/problems/prob03a.txt") {
        for line in lines {
            if let Ok(ip) = line {
               rucksack.push(ip); 
            }
        }
    }
    let x = common_element(rucksack[0].clone());
    println!("Part 1 ans: {}", x);
}

fn common_element(rucksack: String) -> char {
    let char_vec: Vec<char> = rucksack.chars().collect();
    let midpoint: usize = char_vec.len() / 2 - 1;
    let vector_first_half: Vec<char> = char_vec[0..midpoint].to_vec();
    let vector_second_half: Vec<char> = char_vec[midpoint + 1 .. char_vec.len()-1].to_vec();

    let first_half: HashSet<char> = vector_first_half.into_iter().collect();
    let second_half: HashSet<char> = vector_second_half.into_iter().collect();

    let intersection: Vec<&char> = first_half.intersection(&second_half).collect();

    return intersection[0].clone();
}

