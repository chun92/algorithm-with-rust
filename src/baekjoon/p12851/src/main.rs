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

    // (count, step)
    let mut visited = vec![(0, -1); 200001];
    let mut queue: VecDeque<usize> = VecDeque::new();

    let mut step = 0;
    visited[n] = (1, step);
    queue.push_back(n);
    while !queue.is_empty() {
        let mut count = 0;
        let mut next_queue = VecDeque::new();
        let mut hit = false;
        while let Some(v) = queue.pop_front() {
            let current_count = visited[v].0;
            if v == k {
                count += current_count;
                hit = true;
            } else {
                if v > 0 {
                    let (next_count, next_step) = visited[v - 1];
                    if next_step == -1 {
                        next_queue.push_back(v - 1);
                        visited[v - 1] = (current_count + next_count, step + 1);
                    } else if next_step == step + 1 {
                        visited[v - 1] = (current_count + next_count, step + 1);
                    }
                }
                if v < k {
                    let (next_count, next_step) = visited[v + 1];
                    if next_step == -1 {
                        next_queue.push_back(v + 1);
                        visited[v + 1] = (current_count + next_count, step + 1);
                    } else if next_step == step + 1 {
                        visited[v + 1] = (current_count + next_count, step + 1);
                    }
                }
                if v < k {
                    let (next_count, next_step) = visited[v * 2];
                    if next_step == -1 {
                        next_queue.push_back(v * 2);
                        visited[v * 2] = (current_count + next_count, step + 1);
                    } else if next_step == step + 1 {
                        visited[v * 2] = (current_count + next_count, step + 1);
                    }
                }
            }
        }

        if hit {
            println!("{}", step);
            println!("{}", count);
            return;
        }
        queue = next_queue;
        step += 1;
    }
}
