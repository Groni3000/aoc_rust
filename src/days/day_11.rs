#[derive(Debug, Clone)]
struct Monkey<'a> {
    // n: u8,
    items: Vec<usize>,
    operetion_obj: &'a str,
    operation_type: &'a str,
    divisible_by: usize,
    to_wich_monkey_trow: Vec<usize>,
    inspected_items: usize,
}

impl<'a> Monkey<'a> {
    fn run_monkey(&mut self) -> Vec<(usize, usize)> {
        let mut commands = vec![];

        for i in 0..self.items.len() {
            self.inspected_items += 1;
            let mut current_item = self.items[i];
            let operation_object = self.parse_operation_object(&current_item);
            current_item = match self.operation_type {
                "+" => current_item + operation_object,
                "-" => current_item - operation_object,
                "*" => current_item * operation_object,
                "/" => current_item / operation_object,
                _ => unreachable!("Nice operation type maaaan!"),
            };
            current_item = current_item / 3;
            
            if current_item % self.divisible_by == 0 {
                commands.push((current_item, self.to_wich_monkey_trow[0]));
            } else {
                commands.push((current_item, self.to_wich_monkey_trow[1]));
            }
        }
        
        self.items.clear();
        commands
    }

    fn parse_operation_object(&self, current_item: &usize) -> usize {
        match self.operetion_obj.parse::<usize>() {
            Ok(x) => x,
            _ => *current_item,
        }
    }

}

fn read_input() -> Vec<Vec<&'static str>> {
    let input = include_str!(r"D:\rust\exercises\AOC_rust\src\days\inputs\day_11.txt");
    let lines_info = input.lines().collect::<Vec<_>>();
    let chunked_lines_info = lines_info
        .chunks(7)
        .map(|v| v.iter().copied().collect())
        .collect();

    chunked_lines_info
}

pub fn solve_v1(n_rounds: u8) {
    let monkeys_info = read_input();
    let mut monkeys = Vec::with_capacity(8);
    //Parsing to objects
    for i in 0..monkeys_info.len() {
        let starting_items = monkeys_info[i][1]
            .split(": ")
            .last()
            .unwrap()
            .split(", ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut splitted_operetion = monkeys_info[i][2].trim_start().split(' ').rev();
        let operetion_obj = splitted_operetion.next().unwrap();
        let operation_type = splitted_operetion.next().unwrap();
        let divisible_by = monkeys_info[i][3]
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let to_wich_monkey = vec![
            monkeys_info[i][4]
                .split(' ')
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            monkeys_info[i][5]
                .split(' ')
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap(),
        ];

        monkeys.push(Monkey {
            // n: i as u8,
            items: starting_items,
            operetion_obj: operetion_obj,
            operation_type: operation_type,
            divisible_by: divisible_by,
            to_wich_monkey_trow: to_wich_monkey,
            inspected_items: 0,
        })
    }

    //Take a look at them
    // println!("{:?}", monkeys);

    // Rounds started
    for n_round in 0..20 {
        for N in 0..monkeys.len() {
            let monkey_commands = monkeys[N].run_monkey(); //command to trow item and to which monkey
            monkey_commands.iter().for_each(|monkey_command| monkeys[monkey_command.1 as usize].items.push(monkey_command.0));
        }
    }
    println!("-----------");
    println!("{:?}", monkeys);
    // Rounds ended

    // Take 2 max number of inspected numbers
    let mut monkeys_interest = monkeys
        .iter()
        .map(|x| x.inspected_items)
        .collect::<Vec<usize>>();
    monkeys_interest.sort();

    //Print result
    println!(
        "First part solution: {:?}",
        monkeys_interest.into_iter().rev().take(2).reduce(|accum, x| accum * x).unwrap()
    );

}
