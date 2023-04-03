use std::collections::HashSet;

fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn check_adjacent(board: &Vec<Vec<i32>>, set: &mut HashSet<(usize, usize)>, i: usize, j: usize) {
    let m = board.len();
    let n = board[0].len();

    let left = if j == 0 { -1 } else { board[i][j - 1] };
    let right = if j == n - 1 { -1 } else { board[i][j + 1] };
    let up = if i == 0 { -1 } else { board[i - 1][j] };
    let down = if i == m - 1 { -1 } else { board[i + 1][j] };

    if left == 0 {
        set.insert((i, j - 1));
    }

    if right == 0 {
        set.insert((i, j + 1));
    }

    if up == 0 {
        set.insert((i - 1, j));
    } 

    if down == 0 {
        set.insert((i + 1, j));
    }
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0] as usize, v[1] as usize)
    };

    let mut board = vec![vec![0; n]; m];
    for i in 0..m {
        let v = read_line_as_numbers();
        for j in 0..n {
            board[i][j] = v[j];
        }
    }

    let mut ans = 0;
    let mut set = HashSet::new();
    loop {
        let copy = set.clone();

        set.clear();
        if copy.is_empty() {
            for i in 0..m {
                for j in 0..n {
                    if board[i][j] == 1 {
                        check_adjacent(&board, &mut set, i, j);
                    }
                }
            }
        } else {
            for (i, j) in &copy {
                check_adjacent(&board, &mut set, *i, *j);
            }
        }

        if set.is_empty() {
            break;
        } else {
            ans += 1;
            for (i, j) in &set {
                board[*i][*j] = 1;
            }
        }
    }

    for i in 0..m {
        for j in 0..n {
            if board[i][j] == 0 {
                ans = -1;
                break;
            }
        }
    }

    println!("{}", ans);
}
