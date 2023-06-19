fn read_line_as_vec_u8() -> Vec<u8> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().chars().map(|c| c as u8 - '0' as u8).collect()
}

fn return_possible(board: &Vec<Vec<u8>>, row: usize, col: usize) -> Vec<u8> {
    let mut v: Vec<bool> = vec![false; 9];
    for i in 0..9 {
        if board[row][i] == 0 {
            continue;
        }
        v[board[row][i] as usize - 1] = true;
    }
    
    for i in 0..9 {
        if board[i][col] == 0 {
            continue;
        }
        v[board[i][col] as usize - 1] = true;
    }
    
    let row = row / 3 * 3;
    let col = col / 3 * 3;
    for i in 0..3 {
        for j in 0..3 {
            if board[row + i][col + j] == 0 {
                continue;
            }
            v[board[row + i][col + j] as usize - 1] = true;
        }
    }
    v.iter().enumerate().filter(|&(_, &b)| !b).map(|(i, _)| i as u8 + 1).collect()
}

fn solve(board: &mut Vec<Vec<u8>>, row: usize, col: usize) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            if i < row || (i == row && j < col) {
                continue;
            }
            if board[i][j] != 0 {
                continue;
            }
            let v = return_possible(board, i, j);
            if v.len() == 0 {
                return false;
            }
            for k in v {
                board[i][j] = k;
                if solve(board, i, j) {
                    return true;
                }
                board[i][j] = 0;
            }
            return false;
        }
    }
    true
}

fn print_board(board: &Vec<Vec<u8>>) {
    for i in 0..9 {
        for j in 0..9 {
            print!("{}", board[i][j]);
        }
        println!();
    }
}

fn main() {
    let mut board = vec![vec![0; 9]; 9];
    for i in 0..9 {
        let v = read_line_as_vec_u8();
        for j in 0..9 {
            board[i][j] = v[j];
        }
    }

    if solve(&mut board, 0, 0) {
        print_board(&board);
    } else {
        panic!("No solution");
    }
}
