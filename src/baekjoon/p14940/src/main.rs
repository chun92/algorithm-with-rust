use std::{io::{stdin, stdout, BufRead, BufReader, BufWriter, Write, StdinLock}, collections::VecDeque};

fn read_line_as_numbers(stdin: &mut BufReader<StdinLock>) -> Vec<i32> {
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn make_board() -> (Vec<Vec<i32>>, (usize, usize)) {
    let mut stdin = BufReader::new(stdin().lock());
    let (n, m) = {
        let line = read_line_as_numbers(&mut stdin);
        (line[0] as usize, line[1] as usize)
    };

    let mut board = vec![vec![0; m]; n];
    let mut start = (0, 0);
    for i in 0..n {
        let line = read_line_as_numbers(&mut stdin);
        for j in 0..m {
            board[i][j] = line[j];
            if board[i][j] == 2 {
                start = (i, j);
            }
        }
    }
    (board, start)
}

fn fill_board(board: &mut Vec<Vec<i32>>, start: (usize, usize)) {
    let (i, j) = start;
    board[i][j] = 0;

    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut distance = 1;

    let mut visited = vec![vec![false; board[0].len()]; board.len()];

    while !queue.is_empty() {
        let mut next_queue = VecDeque::new();
        while let Some((i, j)) = queue.pop_front() {
            visited[i][j] = true;
            if i > 0 && board[i - 1][j] == 1 && !visited[i - 1][j] {
                board[i - 1][j] = distance;
                next_queue.push_back((i - 1, j));
            }
            if i < board.len() - 1 && board[i + 1][j] == 1 && !visited[i + 1][j]{
                board[i + 1][j] = distance;
                next_queue.push_back((i + 1, j));
            }
            if j > 0 && board[i][j - 1] == 1 && !visited[i][j - 1] {
                board[i][j - 1] = distance;
                next_queue.push_back((i, j - 1));
            }
            if j < board[0].len() - 1 && board[i][j + 1] == 1 && !visited[i][j + 1] {
                board[i][j + 1] = distance;
                next_queue.push_back((i, j + 1));
            }
        }
        queue = next_queue;
        distance += 1;
    }
    
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == 1 && !visited[i][j] {
                board[i][j] = -1;
            }
        }
    }
}

fn print_board(board: &Vec<Vec<i32>>) {
    let mut stdout = BufWriter::new(stdout());
    for i in 0..board.len() {
        let str = board[i]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        writeln!(stdout, "{}", str).unwrap();
    }
}

fn main() {
    let (mut board, start) = make_board();
    fill_board(&mut board, start);
    print_board(&board);
}
