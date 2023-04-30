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

fn bfs(graph: &mut Vec<Vec<Vec<usize>>>, n: usize, m: usize) -> i32 {
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));
    let mut ans = 1;
    let mut visited = vec![vec![vec![false; 2]; m]; n];
    visited[0][0][0] = true;

    while !queue.is_empty() {
        let mut next_queue = VecDeque::new();
        while !queue.is_empty() {
            let (i, j, k) = queue.pop_front().unwrap();
            if i == n - 1 && j == m - 1 {
                return ans as i32;
            }
            if k == 1 {
                if i > 0 && graph[i - 1][j][0] == 0 && !visited[i - 1][j][k] {
                    visited[i - 1][j][k] = true;
                    next_queue.push_back((i - 1, j, k));
                }
                if i < n - 1 && graph[i + 1][j][0] == 0 && !visited[i + 1][j][k] {
                    visited[i + 1][j][k] = true;
                    next_queue.push_back((i + 1, j, k));
                }
                if j > 0 && graph[i][j - 1][0] == 0 && !visited[i][j - 1][k] {
                    visited[i][j - 1][k] = true;
                    next_queue.push_back((i, j - 1, k));
                }
                if j < m - 1 && graph[i][j + 1][0] == 0 && !visited[i][j + 1][k] {
                    visited[i][j + 1][k] = true;
                    next_queue.push_back((i, j + 1, k));
                }
            } else if k == 0 {
                if i > 0 {
                    if graph[i - 1][j][0] == 0 && !visited[i - 1][j][k] {
                        visited[i - 1][j][k] = true;
                        next_queue.push_back((i - 1, j, k));
                    }
                    if graph[i - 1][j][0] == 1 && !visited[i - 1][j][k + 1] {
                        visited[i - 1][j][k + 1] = true;
                        next_queue.push_back((i - 1, j, k + 1));
                    }
                }
                if i < n - 1 {
                    if graph[i + 1][j][0] == 0 && !visited[i + 1][j][k] {
                        visited[i + 1][j][k] = true;
                        next_queue.push_back((i + 1, j, k));
                    }
                    if graph[i + 1][j][0] == 1 && !visited[i + 1][j][k + 1] {
                        visited[i + 1][j][k + 1] = true;
                        next_queue.push_back((i + 1, j, k + 1));
                    }
                }
                if j > 0 {
                    if graph[i][j - 1][0] == 0 && !visited[i][j - 1][k] {
                        visited[i][j - 1][k] = true;
                        next_queue.push_back((i, j - 1, k));
                    }
                    if graph[i][j - 1][0] == 1 && !visited[i][j - 1][k + 1] {
                        visited[i][j - 1][k + 1] = true;
                        next_queue.push_back((i, j - 1, k + 1));
                    }
                }
                if j < m - 1 {
                    if graph[i][j + 1][0] == 0 && !visited[i][j + 1][k] {
                        visited[i][j + 1][k] = true;
                        next_queue.push_back((i, j + 1, k));
                    }
                    if graph[i][j + 1][0] == 1 && !visited[i][j + 1][k + 1] {
                        visited[i][j + 1][k + 1] = true;
                        next_queue.push_back((i, j + 1, k + 1));
                    }
                }
            }
        }
        queue = next_queue;
        ans += 1;
    }
    -1
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut graph = Vec::new();
    for _ in 0..n {
        let s = read_line_as_string();
        let vec = s
            .chars()
            .map(|c| {
                let mut vec = Vec::new();
                vec.push(c.to_digit(10).unwrap() as usize);
                vec.push(0);
                vec
            })
            .collect::<Vec<Vec<usize>>>();
        graph.push(vec);
    }

    let ans = bfs(&mut graph, n, m);
    println!("{}", ans);
}
