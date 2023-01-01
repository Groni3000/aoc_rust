fn compare_me(x: &f32, y: &f32) -> f32 {
    if x >= y {
        return 1.;
    } else {
        return 0.;
    }
}

fn transpose_matrix<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

// fn my_transpose_matrix(matrix: &Vec<Vec<f32>>) -> Vec<Vec<f32>>{
//     let mut new_matrix = <Vec<Vec<f32>> as Clone>::clone(matrix);

//     for i in 0..matrix[0].len() {
//         for j in 0..matrix.len(){
//             new_matrix[i][j] = matrix[j][i];
//         }
//     }

//     new_matrix
// }

fn handler(vector: &[f32], elem: f32, with_rev: bool) -> usize {
    let side_vec = match with_rev {
        true => vector
            .iter()
            .rev()
            .take_while(|x| x < &&&elem)
            .collect::<Vec<_>>(),
        false => vector
            .iter()
            .take_while(|x| x < &&&elem)
            .collect::<Vec<_>>(),
    };
    let side_n = match side_vec.len() == vector.len() {
        true => side_vec.len(),
        false => side_vec.len() + 1,
    };

    side_n
}

pub fn solve_v1() {
    let input = include_str!(r"D:\rust\exercises\AOC_rust\src\days\inputs\day_8.txt");
    let input_lines: Vec<Vec<f32>> = input
        .lines()
        // .take(1)
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<f32>().unwrap())
                .collect::<Vec<f32>>()
        })
        .collect();
    let (n, k) = (input_lines[0].len(), input_lines.len());
    let transposed_input_lines = transpose_matrix(&input_lines);

    let mut sum: u32 = 0;

    for i in 1..input_lines.len() - 1 {
        for j in 1..input_lines[i].len() - 1 {
            let elem = input_lines[i][j];

            let left = input_lines[i][0..j]
                .iter()
                .map(|x| compare_me(&x, &elem))
                .sum::<f32>(); //left
            if left == 0. {
                sum += 1;
                continue;
            }

            let right = input_lines[i][j + 1..]
                .iter()
                .map(|x| compare_me(&x, &elem))
                .sum::<f32>(); //right
            if right == 0. {
                sum += 1;
                continue;
            }

            let up = transposed_input_lines[j][0..i]
                .iter()
                .map(|x| compare_me(&x, &elem))
                .sum::<f32>(); //up
            if up == 0. {
                sum += 1;
                continue;
            }

            let bot = transposed_input_lines[j][i + 1..]
                .iter()
                .map(|x| compare_me(&x, &elem))
                .sum::<f32>(); //bot
            if bot == 0. {
                sum += 1;
                continue;
            }
        }
    }

    println!("First part solution: {}", sum + 4 * ((k - 1) as u32)); //+ 4. * (N-1.)

    let mut scores = Vec::new();

    for i in 1..input_lines.len() - 1 {
        for j in 1..input_lines[i].len() - 1 {
            let elem = input_lines[i][j];
            let left = handler(&input_lines[i][0..j], elem, true);
            let right = handler(&input_lines[i][j + 1..], elem, false);
            let up = handler(&transposed_input_lines[j][0..i], elem, true);
            let bot = handler(&transposed_input_lines[j][i + 1..], elem, false);

            let score = left * right * up * bot;
            scores.push(score);
        }
    }

    println!("Second part solution: {}", scores.iter().max().unwrap());
}
