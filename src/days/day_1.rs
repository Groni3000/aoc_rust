// --- Day 1: Calorie Counting ---
#[allow(unused_variables)]
use std::fs;
use std::vec::Vec;


fn read_input() -> String{
    let file = fs::read_to_string("src/days/inputs/day_1.txt").unwrap();

    return file
}

fn split_input<'a>(input: &'a str) -> Vec<&'a str>{
    return input.split("\r\n").collect::<Vec<&'a str>>()
}

#[derive(Debug)]
pub struct Elf{
    total_calories: u32,
}

pub fn solve_part_1() -> Vec<Elf>{
    let input = read_input();
    let splitted_input = split_input(&input);
    
    let mut elves_population: Vec<Elf> = vec![];
    let mut tmp_vec: Vec<u32> = vec![];

    for number_string in splitted_input {
        match number_string.parse::<u32>(){
            Ok(x) => tmp_vec.push(x),
            Err(e)     => {
                let elf_total_calories = tmp_vec.to_owned().into_iter().sum();
                let new_elf = Elf{total_calories: elf_total_calories};
                elves_population.push(new_elf);
                tmp_vec.clear();
            },
        }
    }

    elves_population.sort_by(|a, b| a.total_calories.partial_cmp(&b.total_calories).unwrap());
    println!("max_calories = {:?}", elves_population[elves_population.len() - 1].total_calories);

    return elves_population
}

pub fn solve_part_2 (elves_population:Vec<Elf>, top_n: u32) {
    println!("Sum of top_{top_n} max_calories = {:?}", elves_population.iter().rev().take(top_n as usize).map(|x| x.total_calories).sum::<u32>());

}

pub fn solve_v2 () {
    let input = include_str!(r"D:\rust\exercises\AOC_rust\src\days\inputs\day_1.txt");

    let mut result: Vec<u32> = input.split("\r\n\r\n")
        .map(|x| {
            return x
            // .split_ascii_whitespace()
            .lines()
            // .map(|x| str::parse::<u32>(x).unwrap())
            .flat_map(str::parse::<u32>)
            .sum();
        })
        .collect();

    result.sort_unstable_by(|a, b| b.cmp(a));

    // println!("solution: {:?}", result);
    println!("First part solution: {:?}", result.first().unwrap());
    println!("Second part solution: {:?}", result.into_iter().take(3).sum::<u32>());
}
