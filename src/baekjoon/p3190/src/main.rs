use std::collections::VecDeque;

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn read_line_as_strings() -> Vec<String> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

fn _print_board(board: &Vec<Vec<usize>>) {
    for row in board {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

fn main() {
    let n = read_line_as_numbers()[0];
    let mut board = vec![vec![0; n]; n];

    let k = read_line_as_numbers()[0];
    for _ in 0..k {
        let xy = read_line_as_numbers();
        board[xy[0] - 1][xy[1] - 1] = 1;
    }

    let l = read_line_as_numbers()[0];
    let mut directions = VecDeque::new();
    for _ in 0..l {
        let line = read_line_as_strings();
        directions.push_back((line[0].parse::<usize>().unwrap(), line[1].chars().next().unwrap()));
    }

    let mut snake = VecDeque::new();
    snake.push_back((0, 0));
    board[0][0] = 2;
    // 0: right, 1: down, 2: left, 3: up
    let mut direction = 0;

    let mut answer = 0;
    loop {
        let (x, y) = snake.front().unwrap();
        let next;
        match direction {
            0 => if *y + 1 < n { next = (*x, *y + 1) } else { break }, // right
            1 => if *x + 1 < n { next = (*x + 1, *y) } else { break }, // down
            2 => if *y > 0 { next = (*x, *y - 1) } else { break }, // left
            3 => if *x > 0 { next = (*x - 1, *y) } else { break }, // up
            _ => panic!("invalid direction")
        }
        let next_value = board[next.0][next.1];

        if next_value == 2 {
            break;
        } else if next_value == 1 {
            snake.push_front(next);
            board[next.0][next.1] = 2;
        } else {
            let tail = snake.pop_back().unwrap();
            board[tail.0][tail.1] = 0;
            snake.push_front(next);
            board[next.0][next.1] = 2;
        }
        answer += 1;

        if let Some((t, d)) = directions.front() {
            if *t == answer {
                match d {
                    'L' => direction = (direction + 3) % 4,
                    'D' => direction = (direction + 1) % 4,
                    _ => panic!("invalid direction")
                }
                directions.pop_front();
            }
        }
    }
    println!("{}", answer + 1);
}
