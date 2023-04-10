use std::collections::VecDeque;

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut a = vec![vec![false; n + 1]; n + 1];
    let mut distance = vec![vec![0; n + 1]; n + 1];
    let mut queue = vec![VecDeque::new(); n + 1];

    for _ in 0..m {
        let (x, y) = {
            let v = read_line_as_numbers();
            (v[0], v[1])
        };
        a[x][y] = true;
        queue[x].push_back(y);
        a[y][x] = true;
        queue[y].push_back(x);
    }

    for i in 1..=n {
        a[i][i] = true;
    }
    
    let mut count = 1;
    loop {
        let mut next_distance = distance.clone();
        let mut changed = false;
        for i in 1..=n {
            let in_queue = &queue[i];
            let mut out_queue = VecDeque::new();
            for j in in_queue {
                if distance[i][*j] == 0 {
                    next_distance[i][*j] = count;
                    changed = true;
                    for k in 1..=n {
                        if k == i || k == *j {
                            continue;
                        }
                        if a[*j][k] && distance[i][k] == 0 {
                            out_queue.push_back(k);
                        }
                    }
                }
            }
            queue[i] = out_queue;
        }
        if !changed {
            break;
        }
        count += 1;
        distance = next_distance;
    }

    let result =
        distance
        .iter()
        .skip(1)
        .map(|v| v.iter().sum::<i32>())
        .collect::<Vec<_>>();

    let min = result.iter().min().unwrap();
    let min_index = result.iter().position(|v| v == min).unwrap() + 1;
    println!("{}", min_index);
}
