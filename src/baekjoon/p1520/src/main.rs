fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn dp(memo: &mut Vec<Vec<usize>>, visited: &mut Vec<Vec<bool>>, graph: &Vec<Vec<usize>>, n: usize, m: usize, i: usize, j: usize) -> usize {
    if visited[i][j] {
        return memo[i][j];
    }

    if i == n - 1 && j == m - 1 {
        return 1;
    }

    visited[i][j] = true;

    let mut sum = 0;
    let cur_num = graph[i][j];
    if i > 0 && cur_num > graph[i - 1][j] {
        sum += dp(memo, visited, graph, n, m, i - 1, j);
    }

    if i < n - 1 && cur_num > graph[i + 1][j] {
        sum += dp(memo, visited, graph, n, m, i + 1, j);
    }

    if j > 0 && cur_num > graph[i][j - 1] {
        sum += dp(memo, visited, graph, n, m, i, j - 1);
    }

    if j < m - 1 && cur_num > graph[i][j + 1] {
        sum += dp(memo, visited, graph, n, m, i, j + 1);
    }

    memo[i][j] = sum;
    memo[i][j]
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut graph = Vec::new();
    for _ in 0..n {
        graph.push(read_line_as_numbers());
    }

    let mut memo = vec![vec![0; m]; n];
    let mut visited = vec![vec![false; m]; n];
    let result = dp(&mut memo, &mut visited, &graph, n, m, 0, 0);
    println!("{}", result);
}
