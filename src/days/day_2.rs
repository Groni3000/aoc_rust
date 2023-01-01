use std::{collections::HashMap};


pub fn solve(){
    //! I dont want to continue this version.

    let input = include_str!(r"D:\rust\exercises\AOC_rust\src\days\inputs\day_2.txt");
    let commands:Vec<&str> = input.lines().collect();
    // println!("{:?}", commands);
    let game_points:HashMap<char, u8> = HashMap::from([
        ('A', 1), ('X', 1), // Rock
        ('B', 2), ('Y', 2), // Paper
        ('C', 3), ('Z', 3), // Scissors
    ]);
    
}

pub fn solve_v1(){
    //? Honestly, I did this because I think that 1 HashMap is the most efficient way to solve this pazzle

    let input = include_str!(r"D:\rust\exercises\AOC_rust\src\days\inputs\day_2.txt");
    let commands:Vec<&str> = input.lines().collect();
    
    let game_outcomes:HashMap<&str, u32> = HashMap::from([
        ("A X", 3+1), ("A Y", 6+2), ("A Z", 0+3), 
        ("B X", 0+1), ("B Y", 3+2), ("B Z", 6+3), 
        ("C X", 6+1), ("C Y", 0+2), ("C Z", 3+3), 
    ]);
    let solution:u32 = commands.iter().map(|x| game_outcomes.get(x).unwrap()).sum();
    println!("First part solution: {:?}", solution);

    let game_outcomes_2:HashMap<&str, u32> = HashMap::from([
        ("A X", 0+3), ("A Y", 3+1), ("A Z", 6+2), 
        ("B X", 0+1), ("B Y", 3+2), ("B Z", 6+3), 
        ("C X", 0+2), ("C Y", 3+3), ("C Z", 6+1), 
    ]);
    let solution:u32 = commands.into_iter().map(|x| game_outcomes_2.get(x).unwrap()).sum();
    println!("Second part solution: {:?}", solution);
}