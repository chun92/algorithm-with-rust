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

fn bfs(graph: &Vec<Vec<usize>>, n: usize, m: usize, h: usize) -> i32 {
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0, 0));
    let mut ans = 1;
    let mut visited = vec![vec![vec![0; 2]; m]; n];
    visited[0][0][0] |= 1 << 15;

    while !queue.is_empty() {
        let mut next_queue = VecDeque::new();
        while !queue.is_empty() {
            let (i, j, k, _l) = queue.pop_front().unwrap();
            if i == n - 1 && j == m - 1 {
                return ans as i32;
            }
            
            let check_and_push = |i: usize, j: usize, k: usize, l: usize,
                visited: &mut Vec<Vec<Vec<u16>>>,
                next_queue: &mut VecDeque<(usize, usize, usize, usize)>,
                break_wall: bool| {
                    let break_wall = if break_wall { 1 } else { 0 };
                    if graph[i][j] != break_wall {
                        return;
                    }

                    let k = if break_wall == 1 { k + 1 } else { k };
                            
                    let mut has_visited = false;
                    let bitmask = 1 << (15 - k);
                    if visited[i][j][l] >= bitmask {
                        has_visited = true;
                    }

                    if !has_visited {
                        visited[i][j][l] |=  bitmask;
                        next_queue.push_back((i, j, k, l));
                    }
            };

            if i > 0 {
                check_and_push(i - 1, j, k, 0, &mut visited, &mut next_queue, false);
            }
            if i < n - 1 {
                check_and_push(i + 1, j, k, 0, &mut visited, &mut next_queue, false);
            }
            if j > 0 {
                check_and_push(i, j - 1, k, 0, &mut visited, &mut next_queue, false);
            }
            if j < m - 1 {
                check_and_push(i, j + 1, k, 0, &mut visited, &mut next_queue, false);
            }
            if k != h && ans % 2 == 1 {
                if i > 0 {
                    check_and_push(i - 1, j, k, 0, &mut visited, &mut next_queue, true);
                }
                if i < n - 1 {
                    check_and_push(i + 1, j, k, 0, &mut visited, &mut next_queue, true);
                }
                if j > 0 {
                    check_and_push(i, j - 1, k, 0, &mut visited, &mut next_queue, true);
                }
                if j < m - 1 {
                    check_and_push(i, j + 1, k, 0, &mut visited, &mut next_queue, true);
                }
            }
            
            if ans % 2 == 0 {                
                let mut has_visited = false;
                let bitmask = 1 << (15 - k);
                if visited[i][j][1] >= bitmask {
                    has_visited = true;
                }

                if !has_visited {
                    visited[i][j][1] |=  bitmask;
                    next_queue.push_back((i, j, k, 1));
                }
            }
        }
        queue = next_queue;
        ans += 1;
    }
    -1
}

fn main() {
    let (n, m, k) = {
        let v = read_line_as_numbers();
        (v[0], v[1], v[2])
    };

    let mut graph = Vec::new();
    for _ in 0..n {
        let s = read_line_as_string();
        let vec = s
            .chars()
            .map(|c| {
                c.to_digit(10).unwrap() as usize
            })
            .collect::<Vec<usize>>();
        graph.push(vec);
    }

    let ans = bfs(&mut graph, n, m, k);
    println!("{}", ans);
}
