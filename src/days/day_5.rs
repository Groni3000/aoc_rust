pub fn solve_v1() {
    let input = include_str!(r"D:\rust\exercises\AOC_rust\src\days\inputs\day_5.txt");
    let crates_str: Vec<&str> = input.lines().take(8).collect();
    let stacks_str: &str = input.lines().nth(8).unwrap();
    let commands_str: Vec<&str> = input.lines().skip(10).collect();

    let crates = crates_str
        .into_iter()
        .map(|x| x.chars().skip(1).step_by(4).collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // println!("{:?}", crates);

    let stacks_ints = stacks_str
        .chars()
        .skip(1)
        .step_by(4)
        .map(|x| x.to_string().parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    let mut stacks: Vec<_> = vec![];

    for stack_n in &stacks_ints {
        let push_this = crates
            .iter()
            .rev()
            .map(|x| x[(stack_n - 1) as usize])
            .take_while(|x| x.is_alphabetic())
            .collect::<Vec<char>>()
            .into_iter()
            // .rev()
            .collect::<Vec<char>>();
        
        stacks.push(push_this);
    }
    // println!("{:?}", stacks);

    let commands = commands_str
        .into_iter()
        // .take(4)
        .map(|x| {
            x.split(' ')
                .skip(1)
                .step_by(2)
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    // println!("{:?}", commands);

    for command in &commands {
        let crates_number = command[0];
        let from = command[1];
        let to = command[2];
        
        for n in 0..crates_number{
            let item = stacks[(from-1) as usize].pop().unwrap();
            stacks[(to-1) as usize].push(item);
        };
    }

    let solution = stacks.iter().map(|x| x.last().unwrap()).collect::<String>();
    println!("Stacks: {:?}", stacks);
    println!("First part solution: {:?}", solution);

    let mut stacks: Vec<_> = vec![];

    for stack_n in stacks_ints {
        let push_this = crates
            .iter()
            .rev()
            .map(|x| x[(stack_n - 1) as usize])
            .take_while(|x| x.is_alphabetic())
            .collect::<Vec<char>>()
            .into_iter()
            // .rev()
            .collect::<Vec<char>>();
        
        stacks.push(push_this);
    }

    for command in commands {
        let crates_number = command[0];
        let from = command[1];
        let to = command[2];
        
        let mut new_vec = Vec::new();
        for n in 0..crates_number{
            let item = stacks[(from-1) as usize].pop().unwrap();
            new_vec.push(item);
        };
        
        new_vec = new_vec.into_iter().rev().collect();

        for item in new_vec{
            stacks[(to-1) as usize].push(item);
        }
    }

    let solution = stacks.iter().map(|x| x.last().unwrap()).collect::<String>();
    println!("Stacks: {:?}", stacks);
    println!("Second part solution: {:?}", solution);
}
