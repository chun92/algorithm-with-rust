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

    let mut visited = vec![-2; 200001];
    let mut queue: VecDeque<usize> = VecDeque::new();

    let mut step = 0;
    visited[n] = -1;
    queue.push_back(n);
    while !queue.is_empty() {
        let mut next_queue = VecDeque::new();
        while let Some(v) = queue.pop_front() {
            if v == k {
                println!("{}", step);
                let mut list = Vec::new();
                let mut prev = k as i32;
                while prev != -1 {
                    list.push(prev);
                    prev = visited[prev as usize];
                }
                list.reverse();
                for (i, v) in list.iter().enumerate() {
                    if i == 0 {
                        print!("{}", v);
                    } else {
                        print!(" {}", v);
                    }
                }
                println!();
                return;
            } else {
                if v < k {
                    if visited[2 * v] == -2 {
                        visited[2 * v] = v as i32;
                        next_queue.push_back(2 * v);
                    }
                }
                if v > 0 {
                    if visited[v - 1] == -2 {
                        visited[v - 1] = v as i32;
                        next_queue.push_back(v - 1);
                    }
                }
                if v < k {
                    if visited[v + 1] == -2 {
                        visited[v + 1] = v as i32;
                        next_queue.push_back(v + 1);
                    }
                }
            }
        }

        queue = next_queue;
        step += 1;
    }
}
