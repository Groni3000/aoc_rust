use std::collections::HashSet;

fn apply_me_1(x:&str) -> u32{
    let chars_iter = x.chars();
    let string_length = chars_iter.to_owned().count();
    let half_length = string_length/2;

    let first_part: HashSet<char> = chars_iter.to_owned().take(half_length).collect();
    let second_part: HashSet<char> = chars_iter.rev().take(half_length).collect();

    // Intersection
    let binding = first_part.intersection(&second_part).collect::<Vec<&char>>();
    let intersection = binding.first().unwrap().to_owned().to_owned(); // <--- WTF IS THAT
    
    // Get points
    let points = match intersection.is_lowercase() {
        true => intersection as u8 - 96,
        false => intersection as u8 - 38,
    };

    return points as u32
}

fn apply_me_2(x:&[&str]) -> u32{
    let mut first_el:HashSet<char> = HashSet::from_iter(x[0].chars());

    for str_chunk in &x[1..]{
        first_el.retain(|item| str_chunk.contains(*item));
    }

    // Get points
    let intersection:Vec<char> = first_el.into_iter().collect();
    let points = match intersection[0].is_lowercase() {
        true => intersection[0] as u8 - 96,
        false => intersection[0] as u8 - 38,
    };

    return points as u32
}

pub fn solve_v1(){
    let input = include_str!(r"D:\rust\exercises\AOC_rust\src\days\inputs\day_3.txt");
    let commands:Vec<&str> = input.lines().collect();
    let solution = commands.iter().map(|x| apply_me_1(x)).sum::<u32>();
    println!("First part solution: {:?}", solution);

    let reshaped_commands = commands.chunks(3);
    let solution: u32 = reshaped_commands.map(|x| apply_me_2(x)).sum();
    println!("Second part solution: {:?}", solution);
}