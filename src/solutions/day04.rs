use std::fs;

type Board<T> = Vec<T>;

fn check_board(board: &Board<bool>) -> bool {
    let row_check = board.chunks(5).map(|row| row.iter().all(|&v| v)).any(|v| v);
    let column_check = (0..5)
        .into_iter()
        .map(|col| board.iter().skip(col).step_by(5).all(|&v| v))
        .any(|v| v);
    row_check || column_check
}

fn mark_boards(num: u32, boards: &Vec<Board<u32>>, markers: &mut Vec<Board<bool>>) {
    // find number we're looking for and mark it on each board
    let indices = boards
        .iter()
        .map(|b| b.iter().position(|&v| v == num));
    for (i, idx) in indices.enumerate() {
        if let Some(board) = idx {
            markers[i][board] = true;
        }
    }
}

fn compute_score(board: &Board<u32>, marked_board: &Board<bool>) -> u32 {
    board
        .iter()
        .enumerate()
        .filter_map(|(idx, item)| (!marked_board[idx]).then(|| item) )
        .sum()
}

//fn print_board<T: std::fmt::Display>(board: &Board<T>) {
//    for (i, row) in board.iter().enumerate() {
//        print!("{:>2} ", row);
//        if (i+1) % 5 == 0 {
//            println!("");
//        }
//    }
//}

pub fn day04(input: &str) -> (String, String) {
    let line_groups = input.split("\n\n").collect::<Vec<&str>>();

    let draw_numbers: Vec<u32> = line_groups[0]
        .split(",")
        .filter_map(|num| num.parse().ok())
        .collect();

    let boards: Vec<Board<u32>> = line_groups[1..]
        .iter()
        .map(|board| {
            board
                .lines()
                .flat_map(|line| {
                    line.split(" ")
                        .filter_map(|num| num.parse().ok())
                        .collect::<Board<u32>>()
                })
                .collect()
        })
        .collect();


    let mut marked_boards: Vec<Board<bool>> = (0..boards.len())
        .into_iter()
        .map(|_| vec![false; 25])
        .collect();

    let mut part_a_solution: Option<u32> = None;

    'outer: for draw_num in &draw_numbers {
        mark_boards(*draw_num, &boards, &mut marked_boards);
        for (idx, marked_board) in marked_boards.iter().enumerate() {
            if check_board(&marked_board) {
                let unmarked_score = compute_score(&boards[idx], &marked_boards[idx]);
                let score = unmarked_score * draw_num;
                part_a_solution = Some(score);
                break 'outer;
            }
        }
    }

    let mut marked_boards: Vec<Board<bool>> = (0..boards.len())
        .into_iter()
        .map(|_| vec![false; 25])
        .collect();

    let mut part_b_solution = 0u32;
    let mut skip: Vec<usize> = vec![];
    for draw_num in &draw_numbers {
        mark_boards(*draw_num, &boards, &mut marked_boards);
        for (idx, marked_board) in marked_boards.iter().enumerate() {
            if check_board(&marked_board) && !skip.contains(&idx) {
                let unmarked_score = compute_score(&boards[idx], &marked_boards[idx]);
                part_b_solution = unmarked_score * draw_num;
                skip.push(idx);
            }
        }
    }

    (part_a_solution.unwrap().to_string(), part_b_solution.to_string())
}
