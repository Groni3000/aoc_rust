fn read_input<'a>() -> Vec<&'a str> {
    let input = include_str!(r"D:\rust\exercises\AOC_rust\src\days\inputs\day_10.txt");
    input.lines().collect::<Vec<_>>()
}

fn draw_pixel(crt_line: &mut Vec<char>, sprite: &Vec<i32>, register_value: &i32) {
    let a: i32 = crt_line.len() as i32 % 40;
    crt_line.push(match sprite.contains(&a) {
        true => '#',
        false => '.',
    })
}

pub fn solve_v1() {
    let lines = read_input();

    let mut cycle_n = 0 as u32;
    let mut register_value: i32 = 1;
    let mut signal_strengths = vec![];
    let mut stop_indeces = (20u32..221 as u32).step_by(40).rev().collect::<Vec<_>>();

    for line in lines.iter() {
        let splitted_line = line.split(" ").collect::<Vec<_>>();

        if cycle_n == *stop_indeces.last().unwrap() {
            signal_strengths.push((stop_indeces.pop().unwrap() as i32) * register_value);
            if stop_indeces.len() == 0 {
                break;
            }
        }

        if splitted_line[0] == "noop" {
            cycle_n += 1;
            if cycle_n == *stop_indeces.last().unwrap() {
                signal_strengths.push((stop_indeces.pop().unwrap() as i32) * register_value);
                if stop_indeces.len() == 0 {
                    break;
                }
            }
        } else if splitted_line[0] == "addx" {
            cycle_n += 1;
            if cycle_n == *stop_indeces.last().unwrap() {
                signal_strengths.push((stop_indeces.pop().unwrap() as i32) * register_value);
                if stop_indeces.len() == 0 {
                    break;
                }
            };
            cycle_n += 1;
            if cycle_n == *stop_indeces.last().unwrap() {
                signal_strengths.push((stop_indeces.pop().unwrap() as i32) * register_value);
                if stop_indeces.len() == 0 {
                    break;
                }
            };
            register_value += splitted_line[1].parse::<i32>().unwrap();
        }
    }

    println!(
        "First part solution: {:?}",
        signal_strengths.into_iter().sum::<i32>()
    );

    let mut sprite: Vec<i32> = vec![0, 1, 2];
    let mut register_value: i32 = 1;
    let mut cycle_n = 0 as u32;
    let mut crt_line: Vec<char> = vec![];

    for line in lines {
        let splitted_line = line.split(" ").collect::<Vec<_>>();

        if cycle_n == 241 {
            break;
        }

        if splitted_line[0] == "noop" {
            cycle_n += 1;
            if cycle_n == 241 {
                break;
            }
            draw_pixel(&mut crt_line, &sprite, &register_value);
        } else if splitted_line[0] == "addx" {
            cycle_n += 1;
            if cycle_n == 241 {
                break;
            }
            draw_pixel(&mut crt_line, &sprite, &register_value);
            cycle_n += 1;
            if cycle_n == 241 {
                break;
            }
            draw_pixel(&mut crt_line, &sprite, &register_value);
            let value_to_add = splitted_line[1].parse::<i32>().unwrap();
            register_value += value_to_add;
            sprite.iter_mut().for_each(|x| *x += value_to_add);
        }
    }

    println!("Second part solution:");
    for crt in crt_line.chunks(40).collect::<Vec<_>>() {
        println!("{:?}", crt);
    }
}
