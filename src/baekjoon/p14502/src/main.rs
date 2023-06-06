use std::collections::VecDeque;

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn check_virus(board: &Vec<Vec<usize>>, n: usize, m: usize) -> usize {
    let mut board = board.clone();
    let mut visited = vec![vec![false; m]; n];
    let mut queue = VecDeque::new();
    for i in 0..n {
        for j in 0..m {
            if visited[i][j] {
                continue;
            }
            if board[i][j] != 2 {
                continue;
            }
            queue.push_back((i, j));
            visited[i][j] = true;
            while let Some((x, y)) = queue.pop_front() {
                if x > 0 && !visited[x - 1][y] && board[x - 1][y] == 0 {
                    queue.push_back((x - 1, y));
                    visited[x - 1][y] = true;
                    board[x - 1][y] = 2;
                }
                if x < n - 1 && !visited[x + 1][y] && board[x + 1][y] == 0 {
                    queue.push_back((x + 1, y));
                    visited[x + 1][y] = true;
                    board[x + 1][y] = 2;
                }
                if y > 0 && !visited[x][y - 1] && board[x][y - 1] == 0 {
                    queue.push_back((x, y - 1));
                    visited[x][y - 1] = true;
                    board[x][y - 1] = 2;
                }
                if y < m - 1 && !visited[x][y + 1] && board[x][y + 1] == 0 {
                    queue.push_back((x, y + 1));
                    visited[x][y + 1] = true;
                    board[x][y + 1] = 2;
                }
            }
        }
    }

    let mut count = 0;
    for i in 0..n {
        for j in 0..m {
            if board[i][j] == 0 {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut board = vec![vec![0; m]; n];
    for i in 0..n {
        let v = read_line_as_numbers();
        for j in 0..m {
            board[i][j] = v[j];
        }
    }

    let mut max = 0;
    for i in 0..n {
        for j in 0..m {
            if board[i][j] != 0 {
                continue;
            }
            board[i][j] = 1;
            for k in i..n {
                for l in 0..m {
                    if k == i && l <= j {
                        continue;
                    }
                    if board[k][l] != 0 {
                        continue;
                    }
                    board[k][l] = 1;
                    for p in k..n {
                        for q in 0..m {
                            if p == k && q <= l {
                                continue;
                            }
                            if board[p][q] != 0 {
                                continue;
                            }
                            board[p][q] = 1;
                            let count = check_virus(&board, n, m);
                            if count > max {
                                max = count;
                            }
                            board[p][q] = 0;
                        }
                    }
                    board[k][l] = 0;
                }
            }
            board[i][j] = 0;
        }
    }
    println!("{}", max);
}
