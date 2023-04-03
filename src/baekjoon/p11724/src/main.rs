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
    
    let mut graph = vec![vec![0; n + 1]; n + 1];

    for _ in 0..m {
        let (a, b) = {
            let v = read_line_as_numbers();
            (v[0], v[1])
        };
        graph[a][b] = 1;
        graph[b][a] = 1;
    }

    let mut visited = vec![false; n + 1];
    let mut sum = 0;
    for i in 1..=n {
        if visited[i] {
            continue;
        }
        let mut stack = vec![i];
        while let Some(node) = stack.pop() {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            for j in 1..=n {
                if graph[node][j] == 1 && !visited[j] {
                    stack.push(j);
                }
            }
        }
        sum += 1;
    }

    println!("{}", sum);
}
