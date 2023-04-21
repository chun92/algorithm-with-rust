use std::collections::VecDeque;

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn create_board() -> Vec<Vec<usize>> {
    let n = read_line_as_numbers()[0];
    let mut board = Vec::new();
    for _ in 0..n {
        board.push(read_line_as_numbers());
    }
    board
}

fn find_nine(board: &Vec<Vec<usize>>) -> (usize, usize) {
    for (i, row) in board.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 9 {
                return (i, j)
            }
        }
    }
    (0, 0)
}

fn bfs(board: &mut Vec<Vec<usize>>, start: (usize, usize), current_size: usize) -> (usize, Option<(usize, usize)>) {
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut distance = 0;
    let mut visited = vec![vec![false; board.len()]; board.len()];

    loop {
        let mut next_queue = VecDeque::new();
        let mut answer_vec = Vec::new();
        while let Some(point) = queue.pop_front() {
            let (i, j) = point;
            if visited[i][j] {
                continue;
            }
            visited[i][j] = true;

            let mut check = |x: usize, y: usize | {
                // println!("({}, {}), ({}, {})", i, j, x, y);
                let target = board[x][y];
                if target != 0 && current_size > target {
                    answer_vec.push((x, y));
                } else if target == 0 || current_size == target {
                    next_queue.push_back((x, y));
                }
            };

            if i > 0 {
                check(i - 1, j);
            }
            if i < board.len() - 1 {
                check(i + 1, j);
            }
            if j > 0 {
                check(i, j - 1);
            }
            if j < board.len() - 1 {
                check(i, j + 1);
            }
        }

        if answer_vec.len() > 0 {
            answer_vec.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
            distance += 1;
            return (distance, Some(answer_vec[0]));
        } else if next_queue.len() == 0 {
            return (distance, None);
        } else {
            queue = next_queue;
            distance += 1;
        }
    }
}

fn solve(board: &mut Vec<Vec<usize>>) -> usize {
    let mut current_size = 2;
    let mut current_point = find_nine(board);
    board[current_point.0][current_point.1] = 0;
    let mut count = 0;
    let mut total_distance = 0;

    loop {
        let (distance, answer) = bfs(board, current_point, current_size);
        match answer {
            Some(point) => {
                board[point.0][point.1] = 0;
                current_point = point;
                count += 1;
                total_distance += distance;
                if current_size == count {
                    current_size += 1;
                    count = 0;
                }
            },
            None => break,
        }
    }

    total_distance
}

fn main() {
    let mut board = create_board();
    println!("{}", solve(&mut board));
}
