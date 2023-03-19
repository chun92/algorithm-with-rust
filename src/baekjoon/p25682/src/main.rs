fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn read_line_as_board(n: usize) -> Vec<Vec<bool>> {
    let mut board = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        board.push(line.trim().chars().map(|c| c == 'B').collect());
    }
    board
}

fn get_sum(board: &Vec<Vec<bool>>, n: usize, m: usize, start: bool) -> Vec<Vec<usize>> {
    let mut col = start;
    let mut row = start;
    let mut result = vec![vec![0; m]; n];
    for i in 0..n {
        let mut sum = 0;
        for j in 0..m {
            if j == 0 {
                row = col;
            }
            if board[i][j] != row {
                sum += 1;
            }
            if i > 0 {
                result[i][j] = result[i - 1][j] + sum;
            } else {
                result[i][j] = sum;
            }
            row = !row;
        }
        col = !col;
    }
    result
}

fn get_partial(sum: &Vec<Vec<usize>>, i: usize, j: usize, k: usize) -> usize {
    if i != 0 && j != 0 {
        sum[i + k - 1][j + k - 1] + sum[i - 1][j - 1] - sum[i - 1][j + k - 1] - sum[i + k - 1][j - 1]
    } else if i != 0 {
        sum[i + k - 1][j + k - 1] - sum[i - 1][j + k - 1]
    } else if j != 0 {
        sum[i + k - 1][j + k - 1] - sum[i + k - 1][j - 1]
    } else {
        sum[i + k - 1][j + k - 1]
    }
}

fn get_min(sum1: &Vec<Vec<usize>>, sum2: &Vec<Vec<usize>>, n: usize, m: usize, k: usize) -> usize {
    let mut min = std::usize::MAX;
    for i in 0..n - k + 1 {
        for j in 0..m - k + 1 {
            let partial1 = get_partial(sum1, i, j, k);
            let partial2 = get_partial(sum2, i, j, k);
            let partial = if partial1 < partial2 { partial1 } else { partial2 };
            if partial < min {
                min = partial;
            }
        }
    }
    min
}

fn main() {
    let args = read_line_as_numbers();
    let (n, m, k) = (args[0], args[1], args[2]);
    let board = read_line_as_board(n);
    let sum1 = get_sum(&board, n, m, true);
    let sum2 = get_sum(&board, n, m, false);
    let min = get_min(&sum1, &sum2, n, m, k);
    println!("{}", min);
}
