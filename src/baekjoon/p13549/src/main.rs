use std::collections::VecDeque;

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (n, k) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut visited = vec![false; 200001];
    let mut queue: VecDeque<usize> = VecDeque::new();

    let mut step = 0;
    visited[n] = true;
    queue.push_back(n);
    while !queue.is_empty() {
        let mut next_queue = VecDeque::new();
        while let Some(v) = queue.pop_front() {
            if v == k {
                println!("{}", step);
                return;
            } else {
                if v < k {
                    if !visited[2 * v] {
                        visited[2 * v] = true;
                        queue.push_back(2 * v);
                    }
                }
                if v > 0 {
                    if !visited[v - 1] {
                        visited[v - 1] = true;
                        next_queue.push_back(v - 1);
                    }
                }
                if v < k {
                    if !visited[v + 1] {
                        visited[v + 1] = true;
                        next_queue.push_back(v + 1);
                    }
                }
            }
        }

        queue = next_queue;
        step += 1;
    }
}
