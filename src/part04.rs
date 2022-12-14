use std::collections::HashSet;

use crate::utility;

pub fn run_part_04() {
    println!("Running Day 04");
    let mut fully_contain = 0;
    let mut overlap: i32 = 0;
    if let Ok(lines) = utility::read_lines("./src/problems/prob04a.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let assignments= ip.split(',')
                                                .map(|x| get_range_vector(x))
                                                .collect::<Vec<Vec<u32>>>();

                let first: HashSet<u32> = assignments.first().unwrap().clone().into_iter().collect();
                let second: HashSet<u32> = assignments.last().unwrap().clone().into_iter().collect();
                

                if first.is_subset(&second) && (first.len() != second.len()) {
                    fully_contain += 1;
                } else if second.is_subset(&first) && (first.len() != second.len()) {
                    fully_contain += 1;
                } else if first.iter().all(|key| second.contains(key)) {
                    fully_contain += 1;
                } else {
                    fully_contain += 0;
                }

                let common_elements = first.intersection(&second).collect::<Vec<&u32>>();
                if common_elements.len() > 0 {
                    overlap = overlap + 1;
                } else {
                    overlap = overlap;
                }
            }
        }

        println!("Part 1 ans: {}", fully_contain);
        println!("Part 2 ans: {}", overlap);
    }
}


fn get_range_vector(input: &str) -> Vec<u32> {
    let x = input.split('-').collect::<Vec<&str>>();
    let first = x.first().unwrap().parse::<u32>().unwrap();
    let second = x.last().unwrap().parse::<u32>().unwrap();

    (first..=second).collect()
}