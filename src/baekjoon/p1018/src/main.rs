use std::usize::MAX;

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn read_line_as_board(n: usize, m: usize) -> Vec<Vec<bool>> {
    let mut result = vec![vec![false; m]; n];
    for i in 0..n {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().chars().enumerate().for_each(|(j, c)| {
            if c == 'B' {
                result[i][j] = false;
            } else if c == 'W' {
                result[i][j] = true;
            }
        });
    }
    result
}

fn read_num_to_fix(board: &Vec<Vec<bool>>, n: usize, m: usize) -> usize {
    let first = true;
    let second = false;

    let mut result = 0;
    for i in n..n + 8 {
        for j in m..m + 8 {
            if (i + j - n - m) % 2 == 0 {
                if board[i][j] != first {
                    result += 1;
                }
            } else {
                if board[i][j] != second {
                    result += 1;
                }
            }
        }
    }
    std::cmp::min(result, 64 - result)
}

fn main() {
    let args = read_line_as_numbers();
    let (n, m) = (args[0], args[1]);
    let board = read_line_as_board(n, m);

    let mut result = MAX;
    for i in 0..n - 7 {
        for j in 0..m - 7 {
            let num = read_num_to_fix(&board, i, j);
            result = std::cmp::min(result, num);
        }
    }
    println!("{}", result);
}
