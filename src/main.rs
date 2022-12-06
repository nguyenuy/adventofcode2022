use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let mut calories: Vec<Vec<i32>> = Vec::new(); 
    let mut _elf_calories: Vec<i32> = Vec::new();

    // Build a vector of elf vector calories
    if let Ok(lines) = read_lines("./src/prob01.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    calories.push(_elf_calories.clone());
                    _elf_calories = Vec::new();
                } else {
                    let _calorie = ip.parse::<i32>().unwrap();
                    _elf_calories.push(_calorie)
                }
            }
        }
    }

    let mut acc = calories[0].clone();
    
    let mut calories_per_elf = calories
                                    .iter()
                                    .map(|x| {x.iter().sum()})
                                    .collect::<Vec<i32>>();
        
    let mut part_1_ans = calories_per_elf.clone().iter().max();

    let mut part_2_ans = calories_per_elf.sort();
    let sum: i32 = calories[0].clone().iter().sum();

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}