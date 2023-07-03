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

fn dfs(board: &Vec<Vec<char>>, memo: &mut Vec<Vec<Vec<Vec<usize>>>>, red: (usize, usize), blue: (usize, usize), dir: usize, depth: usize) -> (bool, usize) {
    let (red, blue, success_red, success_blue) = move_ball(board, red, blue, dir);
    
    if memo[red.0][red.1][blue.0][blue.1] <= depth {
        return (false, usize::MAX);
    }
    // println!("depth: {}, direction: {}, red({}, {}), blue({}, {})", depth, dir, red.0, red.1, blue.0, blue.1);
    memo[red.0][red.1][blue.0][blue.1] = depth;

    if success_red && !success_blue {
        return (true, depth + 1);
    }
    if success_blue {
        return (false, usize::MAX);
    }

    let mut min_depth = usize::MAX;
    for i in 0..4 {
        if i == dir {
            continue;
        }
        let (success, depth) = dfs(board, memo, red, blue, i, depth + 1);
        if success {
            min_depth = std::cmp::min(min_depth, depth);
        }
    }

    if min_depth == usize::MAX {
        (false, usize::MAX)
    } else {
        (true, min_depth)
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

    let mut memo = vec![vec![vec![vec![usize::MAX; m]; n]; m]; n];
    let result1 = dfs(&board, &mut memo, red, blue, 0, 0);
    memo[red.0][red.1][blue.0][blue.1] = usize::MAX;
    let result2 = dfs(&board, &mut memo, red, blue, 1, 0);
    memo[red.0][red.1][blue.0][blue.1] = usize::MAX;
    let result3 = dfs(&board, &mut memo, red, blue, 2, 0);
    memo[red.0][red.1][blue.0][blue.1] = usize::MAX;
    let result4 = dfs(&board, &mut memo, red, blue, 3, 0);
    let min = std::cmp::min(
        std::cmp::min(result1.1, result2.1),
        std::cmp::min(result3.1, result4.1),
    );

    if min == usize::MAX {
        println!("-1");
    } else {
        println!("{}", min);
    }
}
