fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn dp(graph: &Vec<Vec<usize>>, memo: &mut Vec<Vec<usize>>, visited: &mut Vec<Vec<bool>>, n: usize, i: usize, j: usize) -> usize {
    if visited[i][j] {
        return memo[i][j];
    }

    let mut res = 0;
    visited[i][j] = true;
    let current = graph[i][j];

    if i > 0 && current < graph[i - 1][j] {
        res = std::cmp::max(res, dp(graph, memo, visited, n, i - 1, j) + 1);
    }

    if j > 0 && current < graph[i][j - 1] {
        res = std::cmp::max(res, dp(graph, memo, visited, n, i, j - 1) + 1);
    }

    if i < n - 1 && current < graph[i + 1][j] {
        res = std::cmp::max(res, dp(graph, memo, visited, n, i + 1, j) + 1);
    }

    if j < n - 1 && current < graph[i][j + 1] {
        res = std::cmp::max(res, dp(graph, memo, visited, n, i, j + 1) + 1);
    }

    memo[i][j] = res;
    res
}

fn main() {
    let n = read_line_as_numbers()[0];

    let mut graph = Vec::new();
    for _ in 0..n {
        graph.push(read_line_as_numbers());
    }

    let mut memo = vec![vec![0; n]; n];
    let mut visited = vec![vec![false; n]; n];

    for i in 0..n {
        for j in 0..n {
            dp(&graph, &mut memo, &mut visited, n, i, j);
        }
    }

    let mut res = 0;
    let mut indexes = (0, 0);
    for i in 0..n {
        for j in 0..n {
            if memo[i][j] > res {
                res = memo[i][j];
                indexes = (i, j);
            }
        }
    }

    println!("{}, ({}, {})", res, indexes.0, indexes.1);
}
