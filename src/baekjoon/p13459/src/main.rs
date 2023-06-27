use std::collections::VecDeque;

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

    let mut queue = VecDeque::new();
    queue.push_back((red, blue, 0));
    queue.push_back((red, blue, 1));
    queue.push_back((red, blue, 2));
    queue.push_back((red, blue, 3));

    for _ in 1..=10 {
        let mut next_queue = VecDeque::new();
        while let Some((red, blue, dir)) = queue.pop_front() {
            let (red, blue, success_red, success_blue) = move_ball(&board, red, blue, dir);
            if success_red && !success_blue {
                println!("1");
                return;
            }
            if success_blue {
                continue;
            }
            next_queue.push_back((red, blue, 0));
            next_queue.push_back((red, blue, 1));
            next_queue.push_back((red, blue, 2));
            next_queue.push_back((red, blue, 3));
        }
        queue = next_queue;
    }

    println!("0");
}
