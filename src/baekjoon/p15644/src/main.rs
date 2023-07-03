fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn read_line_as_string() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn move_ball_one(board: &Vec<Vec<char>>, ball: (usize, usize), other: (usize, usize), dir: usize) -> ((usize, usize), bool) {
    let mut ball = ball;
    let mut success = false;

    loop {
        let (x, y) = ball;
        let (next_x, next_y) = match dir {
            0 /* left */ => (x, y - 1),
            1 /* up */ => (x - 1, y),
            2 /* right */ => (x, y + 1),
            3 /* down */ => (x + 1, y),
            _ => panic!("Invalid direction"),
        };

        if (next_x, next_y) == other && board[next_x][next_y] == '.' {
            break;
        }

        match board[next_x][next_y] {
            '.' => {
                ball = (next_x, next_y);
                continue;
            },
            '#' => {
                break;
            },
            'O' => {
                ball = (next_x, next_y);
                success = true;
                break;
            },
            _ => panic!("Invalid character"),
        }
    }

    (ball, success)
}

fn move_ball(board: &Vec<Vec<char>>, red: (usize, usize), blue: (usize, usize), dir: usize) -> ((usize, usize), (usize, usize), bool, bool) {
    match dir {
        0 /* left */ => {
            if red.1 < blue.1 {
                let (red, success_red) = move_ball_one(board, red, blue, dir);
                let (blue, success_blue) = move_ball_one(board, blue, red, dir);
                (red, blue, success_red, success_blue)
            } else {
                let (blue, success_blue) = move_ball_one(board, blue, red, dir);
                let (red, success_red) = move_ball_one(board, red, blue, dir);
                (red, blue, success_red, success_blue)
            }
        },
        1 /* up */ => {
            if red.0 < blue.0 {
                let (red, success_red) = move_ball_one(board, red, blue, dir);
                let (blue, success_blue) = move_ball_one(board, blue, red, dir);
                (red, blue, success_red, success_blue)
            } else {
                let (blue, success_blue) = move_ball_one(board, blue, red, dir);
                let (red, success_red) = move_ball_one(board, red, blue, dir);
                (red, blue, success_red, success_blue)
            }
        },
        2 /* right */ => {
            if red.1 < blue.1 {
                let (blue, success_blue) = move_ball_one(board, blue, red, dir);
                let (red, success_red) = move_ball_one(board, red, blue, dir);
                (red, blue, success_red, success_blue)
            } else {
                let (red, success_red) = move_ball_one(board, red, blue, dir);
                let (blue, success_blue) = move_ball_one(board, blue, red, dir);
                (red, blue, success_red, success_blue)
            }
        },
        3 /* down */ => {
            if red.0 < blue.0 {
                let (blue, success_blue) = move_ball_one(board, blue, red, dir);
                let (red, success_red) = move_ball_one(board, red, blue, dir);
                (red, blue, success_red, success_blue)
            } else {
                let (red, success_red) = move_ball_one(board, red, blue, dir);
                let (blue, success_blue) = move_ball_one(board, blue, red, dir);
                (red, blue, success_red, success_blue)
            }
        },
        _ => panic!("Invalid direction"),
    }
}

fn dfs(board: &Vec<Vec<char>>, red: (usize, usize), blue: (usize, usize), dir: usize, depth: usize, str: usize) -> (bool, usize, usize) {
    if depth == 11 {
        return (false, 11, str);
    }

    let (red, blue, success_red, success_blue) = move_ball(board, red, blue, dir);
    if success_red && !success_blue {
        return (true, depth + 1, str);
    }
    if success_blue {
        return (false, 11, str);
    }

    let mut min_depth = 11;
    let mut min_str = str;
    for i in 0..4 {
        if i == dir {
            continue;
        }
        let new_str = str << 2 | i;
        let (success, depth, result_str) = dfs(board, red, blue, i, depth + 1, new_str);
        if success {
            if min_depth > depth {
                min_depth = depth;
                min_str = result_str;
            }
        }
    }

    if min_depth == 11 {
        (false, 11, min_str)
    } else {
        (true, min_depth, min_str)
    }
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut board = vec![vec![' '; m]; n];

    let mut red = (0, 0);
    let mut blue = (0, 0);
    for i in 0..n {
        let line = read_line_as_string();
        for (j, c) in line.chars().enumerate() {
            board[i][j] = c;
            if c == 'R' {
                red = (i, j);
                board[i][j] = '.';
            } else if c == 'B' {
                blue = (i, j);
                board[i][j] = '.';
            }
        }
    }

    let result1 = dfs(&board, red, blue, 0, 0, 0);
    let result2 = dfs(&board, red, blue, 1, 0, 1);
    let result3 = dfs(&board, red, blue, 2, 0, 2);
    let result4 = dfs(&board, red, blue, 3, 0, 3);

    let min = std::cmp::min(
        std::cmp::min(result1.1, result2.1),
        std::cmp::min(result3.1, result4.1),
    );

    let string = if min == result1.1 {
        result1.2
    } else if min == result2.1 {
        result2.2
    } else if min == result3.1 {
        result3.2
    } else {
        result4.2
    };

    if min == 11 {
        println!("-1");
    } else {
        println!("{}", min);
        let mut result = Vec::new();
        for i in 0..min {
            let dir = (string >> (2 * (min - i - 1))) & 3;
            result.push(match dir {
                0 => 'L',
                1 => 'U',
                2 => 'R',
                3 => 'D',
                _ => panic!("Invalid direction"),
            });
        }
        println!("{}", result.iter().collect::<String>());
    }
}
