#[derive(PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

struct RelPosition {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

impl Point {
    fn new() -> Point {
        Point { x: 0, y: 0 }
    }

    fn move_point(&mut self, direction: &str) {
        match direction {
            "R" => self.x += 1,
            "L" => self.x -= 1,
            "U" => self.y += 1,
            "D" => self.y -= 1,
            _ => unreachable!("Nice input"),
        }
    }

    fn is_near(&self, other: &Point) -> bool {
        if (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1 {
            true
        } else {
            false
        }
    }

    fn check_rel_position(&self, other: &Point) -> RelPosition {
        RelPosition {
            left: self.x < other.x,
            right: self.x > other.x,
            up: self.y > other.y,
            down: self.y < other.y,
        }
    }
    
    fn fix_pos(&mut self, rel_pos:&RelPosition) {
        if rel_pos.left {self.move_point("R")};
        if rel_pos.right {self.move_point("L")};
        if rel_pos.up {self.move_point("D")};
        if rel_pos.down {self.move_point("U")};
    }

}

fn read_input<'a>() -> Vec<(&'a str, i32)> {
    let input = include_str!(r"D:\rust\exercises\AOC_rust\src\days\inputs\day_9.txt");

    let input_lines: Vec<(&str, i32)> = input
        .lines()
        .map(|line| {
            let mut values = line.split(" ");
            (
                values.next().unwrap(),
                values.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect();

    input_lines
}

pub fn solve_v1(n:usize) {
    let input_lines: Vec<(&str, i32)> = read_input();

    // let (mut head, mut tail) = (Point::new(), Point::new());
    // let mut seen_points: Vec<Point> = vec![];
    
    // for line in input_lines.iter() {
    //     let (direction, n_of_moves) = (line.0, line.1);
    //     for i in 0..n_of_moves {
    //         head.move_point(direction);
    //         if !tail.is_near(&head){
    //             let rel_pos = tail.check_rel_position(&head);
    //             tail.fix_pos(&rel_pos);
    //         }
            
    //         if !seen_points.contains(&tail){
    //             seen_points.push(tail);
    //         }
    //     }
    // }
    
    // println!("First part solution: {}", seen_points.len());
    
    let mut points = vec![];
    for i in 0..n {points.push(Point::new())}; //0 - head, -1 - tail
    let mut seen_points: Vec<Point> = vec![];
    
    for line in input_lines.iter() {
        let (direction, n_of_moves) = (line.0, line.1);
        for i in 0..n_of_moves {
            points[0].move_point(direction);
            
            for i in 1..n {
                if !points[i].is_near(&points[i-1]){
                    let rel_pos = points[i].check_rel_position(&points[i-1]);
                    points[i].fix_pos(&rel_pos);
                }else{continue;}
            }
            
            if !seen_points.contains(&points[n-1]){
                seen_points.push(points[n-1]);
            }
        }
    }
    
    println!("Solution: {}", seen_points.len());
}
