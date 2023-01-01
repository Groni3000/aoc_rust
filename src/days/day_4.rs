pub fn solve_v1() {
    let input = include_str!(r"D:\rust\exercises\AOC_rust\src\days\inputs\day_4.txt");
    let commands: Vec<Vec<Vec<u32>>> = input
        .lines()
        .into_iter()
        // .take(20)
        .map(|line| {
            line.split(",")
                .map(|inner_str| {
                    inner_str
                        .split("-")
                        .collect::<Vec<&str>>()
                        .into_iter()
                        .map(|numeric_str| numeric_str.parse::<u32>().unwrap()).collect()
                })
                .collect()
        })
        .collect();
    
    let mut counter: u32 = 0;
    
    for pair in commands.to_owned(){
        if pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][1]{
            counter += 1;
        } else if pair[0][0] >= pair[1][0] && pair[0][1] <= pair[1][1] {
            counter += 1;
        }
    };
    println!("First part solution: {:?}", counter);
    
    let mut counter: u32 = 0;

    for pair in commands{
        let val_10_inside = pair[1][0] >= pair[0][0] && pair[1][0] <= pair[0][1];
        let val_11_inside = pair[1][1] >= pair[0][0] && pair[1][1] <= pair[0][1];
        let val_00_inside = pair[0][0] >= pair[1][0] && pair[0][0] <= pair[1][1];
        let val_01_inside = pair[0][1] >= pair[1][0] && pair[0][1] <= pair[1][1];

        if val_10_inside || val_11_inside || val_00_inside || val_01_inside{
            counter += 1;
        }
    };
    println!("Second part solution: {:?}", counter);

}
