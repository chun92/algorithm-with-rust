use std::io::{stdin, BufRead};

fn bfs(n: usize, l: i32, r: i32, board: &mut Vec<Vec<i32>>, x: usize, y: usize, visited: &mut Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    let dx = [0, 0, -1, 1];
    let dy = [-1, 1, 0, 0];
    
    let mut countries = vec![(x, y)];
    let mut queue = vec![(x, y)];
    visited[x][y] = true;

    while let Some((x, y)) = queue.pop() {
        for i in 0..4 {
            let nx = x as i32 + dx[i];
            let ny = y as i32 + dy[i];
            if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 {
                let nx = nx as usize;
                let ny = ny as usize;
                if !visited[nx][ny] && l <= (board[x][y] - board[nx][ny]).abs() && (board[x][y] - board[nx][ny]).abs() <= r {
                    queue.push((nx, ny));
                    visited[nx][ny] = true;
                    countries.push((nx, ny));
                }
            }
        }
    }
    countries
}

fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();

    let line = lines.next().unwrap().unwrap();
    let mut split = line.split_whitespace();
    let n: usize = split.next().unwrap().parse().unwrap();
    let l: i32 = split.next().unwrap().parse().unwrap();
    let r: i32 = split.next().unwrap().parse().unwrap();

    let mut board: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        board.push(row);
    }

    let mut total_days = 0;
    loop {
        let mut visited = vec![vec![false; n]; n];
        let mut changed = false;
        for i in 0..n {
            for j in 0..n {
                if !visited[i][j] {
                    let countries = bfs(n, l, r, &mut board, i, j, &mut visited);
                    let total_population: i32 = countries.iter().map(|&(x, y)| board[x][y]).sum();
                    let average_population = total_population / countries.len() as i32;
                    for &(x, y) in &countries {
                        if board[x][y] != average_population {
                            changed = true;
                        }
                        board[x][y] = average_population;
                    }
                }
            }
        }
        if !changed {
            break;
        }
        total_days += 1;
    }
    println!("{}", total_days);
}
