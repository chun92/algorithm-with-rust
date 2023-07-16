fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_numbers()[0];
    let (x, y) = {
        let v = read_line_as_numbers();
        (v[0] - 1, v[1] - 1)
    };
    let edges = read_line_as_numbers()[0];

    let mut graph = vec![vec![false; n]; n];
    for _ in 0..edges {
        let (x, y) = {
            let v = read_line_as_numbers();
            (v[0] - 1, v[1] - 1)
        };
        graph[x][y] = true;
        graph[y][x] = true;
    }

    let mut queue = Vec::new();
    let mut visited = vec![false; n];

    queue.push(x);
    visited[x] = true;
    let mut count = 1;

    loop {
        if queue.is_empty() {
            break;
        }
        let mut new_queue = Vec::new();
        for &node in &queue {
            for (i, &is_connected) in graph[node].iter().enumerate() {
                if is_connected && !visited[i] {
                    if i == y {
                        println!("{}", count);
                        return;
                    }
                    new_queue.push(i);
                    visited[i] = true;
                }
            }
        }
        queue = new_queue;
        count += 1;
    }

    println!("-1");
}
