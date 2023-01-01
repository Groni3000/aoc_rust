use std::num::ParseIntError;

fn file_handler(
    stack: &mut Vec<(&str, usize)>,
    line_command: &Vec<&str>,
    number: Result<usize, ParseIntError>,
) {
    let file_inside = &line_command[1];
    let file_size = number.unwrap();
    stack.iter_mut().for_each(|path| path.1 += file_size);
}

fn cd_out_handler<'a>(stack: &mut Vec<(&'a str, usize)>, paths: &mut Vec<(&'a str, usize)>) {
    let last_folder = stack.pop().unwrap();
    paths.push(last_folder);
}

fn cd_in_handler<'a>(stack: &mut Vec<(&'a str, usize)>, new_folder: &'a str) {
    stack.push((new_folder, 0));
}

pub fn solve_v1() {
    let input = include_str!(r"D:\rust\exercises\AOC_rust\src\days\inputs\day_7.txt");
    let input_lines = input.lines().collect::<Vec<_>>();
    let input_commands = input_lines
        .iter()
        .map(|x| x.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut stack: Vec<(&str, usize)> = vec![("/", 0)];
    let mut paths: Vec<(&str, usize)> = vec![];

    for line_command in &input_commands[2..] {
        if line_command[0] == "$" && line_command[1] == "cd" && line_command[2] == ".." {cd_out_handler(&mut stack, &mut paths);} 
        else if line_command[0] == "$" && line_command[1] == "cd" {cd_in_handler(&mut stack, &line_command[2]);} 
        else {
            let number = line_command[0].parse::<usize>();
            if number.is_ok() {
                file_handler(&mut stack, line_command, number);
            };
        };
    }

    for el in stack.iter() {
        paths.push(*el);
    }

    let sum = paths.iter().filter(|x| x.1 <= 100000 as usize).fold(0, |acc, x| acc + x.1);
    println!("First part solution: {}", sum);

    let total_disk_space: u32 = 70_000_000;
    let need_free_space: u32 = 30_000_000;

    let delta = total_disk_space - stack[0].1 as u32;
    let free: u32 = need_free_space - delta;

    paths.sort_by(|x, y| x.1.cmp(&y.1));
    let folder_to_delete = paths.into_iter().skip_while(|x| x.1 < free as usize).next().unwrap();
    println!("Second part solution: {:?}", folder_to_delete.1);

}
