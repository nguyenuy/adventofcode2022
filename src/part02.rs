use crate::utility;

pub fn run_part_02a() {
    println!("Running Day 02 - Part A");
    let mut opponent_choice: Vec<char> = Vec::new();
    let mut my_choice: Vec<char> = Vec::new();

    // Build the problem vectors
    if let Ok(lines) = utility::read_lines("./src/problems/prob02a.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let choices: Vec<&str> = ip.split(' ').collect();
                match choices[0] {
                    "A" => opponent_choice.push('R'),
                    "B" => opponent_choice.push('P'),
                    "C" => opponent_choice.push('S'),
                    _ => println!("No Match"),
                }
                match choices[1] {
                    "X" => my_choice.push('R'),
                    "Y" => my_choice.push('P'),
                    "Z" => my_choice.push('S'),
                    _ => println!("No Match"),
                }
                
            }
        }
    }

    let mut total_score: i32 = 0;

    for index in 0..my_choice.len() {
        let my_round_score = determine_round_score(opponent_choice[index], my_choice[index]);
        let my_choice_score: i32 = match my_choice[index] {
            'R' => 1,
            'P' => 2,
            'S' => 3,
            _ => 0,
        };

        total_score = total_score + my_round_score + my_choice_score;
    }

    println!("Part 1 ans: {}", total_score);
}

pub fn run_part_02b() {
    println!("Running Day 02 - Part B");
    let mut opponent_choice: Vec<char> = Vec::new();
    let mut my_choice: Vec<char> = Vec::new();

    // Build the problem vectors
    if let Ok(lines) = utility::read_lines("./src/problems/prob02a.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let choices: Vec<&str> = ip.split(' ').collect();
                match choices[0] {
                    "A" => opponent_choice.push('R'),
                    "B" => opponent_choice.push('P'),
                    "C" => opponent_choice.push('S'),
                    _ => println!("No Match"),
                }
                match choices[1] {
                    "X" => {
                        // Opponent Wins
                        match *opponent_choice.last().unwrap() {
                            'R' => my_choice.push('S'),
                            'S' => my_choice.push('P'),
                            'P' => my_choice.push('R'),
                            _ => println!("Something's not right"),
                        }
                    },
                    "Y" => {
                        // Draw
                        my_choice.push(*opponent_choice.last().unwrap());
                    },
                    "Z" => {
                        // Opponent Loses
                        match *opponent_choice.last().unwrap() {
                            'R' => my_choice.push('P'),
                            'S' => my_choice.push('R'),
                            'P' => my_choice.push('S'),
                            _ => println!("Something's not right"),
                        }
                    },
                    _ => println!("No Match"),
                }
                
            }
        }
    }

    let mut total_score: i32 = 0;

    for index in 0..my_choice.len() {
        let my_round_score = determine_round_score(opponent_choice[index], my_choice[index]);
        let my_choice_score: i32 = match my_choice[index] {
            'R' => 1,
            'P' => 2,
            'S' => 3,
            _ => 0,
        };

        total_score = total_score + my_round_score + my_choice_score;
    }

    println!("Part 2 ans: {}", total_score);
}

fn determine_round_score(opp_choice: char, my_choice: char) -> i32 {
    if opp_choice == my_choice {
        return 3;
    } else {
        if opp_choice == 'R' && my_choice == 'P' {
            return 6;
        } else if opp_choice == 'S' && my_choice == 'R' {
            return 6;
        } else if opp_choice == 'P' && my_choice == 'S' {
            return 6;
        } else {
            return 0; // LOST
        }
    }
}