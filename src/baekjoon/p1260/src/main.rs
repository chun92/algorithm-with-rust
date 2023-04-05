fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn dfs(graph: &Vec<Vec<bool>>, visited: &mut Vec<bool>, current: usize, result: &mut Vec<usize>) {
    visited[current] = true;
    result.push(current);
    for (i, &connected) in graph[current].iter().enumerate() {
        if connected && !visited[i] {
            dfs(graph, visited, i, result);
        }
    }
}

fn bfs(graph: &Vec<Vec<bool>>, visited: &mut Vec<bool>, initial: usize, result: &mut Vec<usize>) {
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(initial);
    visited[initial] = true;
    while let Some(current) = queue.pop_front() {
        result.push(current);
        for (i, &connected) in graph[current].iter().enumerate() {
            if connected && !visited[i] {
                queue.push_back(i);
                visited[i] = true;
            }
        }
    }
}

fn main() {
    let (n, m, initial) = {
        let line = read_line_as_numbers();
        (line[0], line[1], line[2])
    };

    let mut graph = vec![vec![false; n + 1]; n + 1];
    for _ in 0..m {
        let (a, b) = {
            let line = read_line_as_numbers();
            (line[0], line[1])
        };
        graph[a][b] = true;
        graph[b][a] = true;
    }

    let mut dfs_visited = vec![false; n + 1];
    let mut dfs_result = Vec::new();
    dfs(&graph, &mut dfs_visited, initial, &mut dfs_result);

    let mut bfs_visited = vec![false; n + 1];
    let mut bfs_result = Vec::new();
    bfs(&graph, &mut bfs_visited, initial, &mut bfs_result);

    println!(
        "{}",
        dfs_result
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
    println!(
        "{}",
        bfs_result
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}