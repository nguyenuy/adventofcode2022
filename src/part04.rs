use std::collections::HashSet;

use crate::utility;

pub fn run_part_03() {
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

    let sum_priorities: u32 = rucksack
                            .clone()
                            .iter()
                            .map(|x| determine_element_priority(common_element(x.clone())))
                            .into_iter()
                            .sum();

    println!("Part 1 ans: {}", sum_priorities);


    let sum_priorities_part_2: u32 = rucksack
                                            .chunks(3)
                                            .into_iter()
                                            .map(|x| determine_element_priority(common_element_group(x.to_vec())))
                                            .into_iter()
                                            .sum();
                        
    println!("Part 2 ans: {}", sum_priorities_part_2);

}

fn common_element(rucksack: String) -> char {
    let char_vec: Vec<char> = rucksack.chars().collect();
    let midpoint: usize = char_vec.len() / 2;
    let vector_first_half: Vec<char> = char_vec[0..midpoint].to_vec();
    let vector_second_half: Vec<char> = char_vec[midpoint .. char_vec.len()].to_vec();

    let first_half: HashSet<char> = vector_first_half.into_iter().collect();
    let second_half: HashSet<char> = vector_second_half.into_iter().collect();

    let intersection: Vec<&char> = first_half.intersection(&second_half).collect();
    intersection[0].clone()
}

fn common_element_group(rucksacks: Vec<String>) -> char {
    let mut intersect_result: Vec<char> = rucksacks[0].chars().collect();

    for sack in rucksacks {
        let char_vec: Vec<char> = sack.chars().collect();
        let char_set: HashSet<char> = char_vec.into_iter().collect();
        intersect_result = char_set
            .intersection(&intersect_result.into_iter().collect())
            .map(|x| *x)
            .collect();                
    }

    intersect_result[0].clone()
}

fn determine_element_priority(element: char) -> u32 {
    if element.is_lowercase() {
        element as u32 - 96
    } else {
        element as u32 - 38
    }
}

