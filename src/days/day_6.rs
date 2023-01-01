

pub fn solve_v1(n:usize) {
    let input = include_str!(r"D:\rust\exercises\AOC_rust\src\days\inputs\day_6.txt");
    let c_storage = input.chars().collect::<Vec<char>>();
    let mut index: usize = 0;
    for i in n..c_storage.len() {
        let mut last_series = c_storage[i-n..i].to_vec();
        let last_signal = &c_storage[i];
        
        last_series.sort();
        last_series.dedup();
        
        if last_series.len() < n || last_series.contains(last_signal){
            continue;
        }

        index = i;
        break;
    }
    println!("First part solution: {:?}", &c_storage[index-n..index + 1]);
    println!("First part solution: {:?}", index + 1);
}