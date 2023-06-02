use std::io::{stdin, stdout, BufWriter, Write};

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn dfs(v: usize, graph: &Vec<Vec<(usize, f64, f64)>>, memo: &mut Vec<f64>, k: f64) -> f64 {
    if memo[v] != f64::MAX {
        return memo[v];
    }

    if v == 0 {
        memo[v] = 0.0;
        return memo[v];
    }
    
    let mut min = f64::MAX;
    for (u, e, d) in &graph[v] {
        let f = d * (e - k);
        let effort = dfs(*u, graph, memo, k) + f;
        if effort < min {
            min = effort;
        }
    }

    memo[v] = min;
    memo[v]
}

const EPS: f64 = 0.0001;

fn binary_search(left: f64, right: f64, graph: &Vec<Vec<(usize, f64, f64)>>) -> f64 {
    let mut left = left;
    let mut right = right;
    let mut mid = (left + right) / 2.0;

    while right - left > EPS {
        let mut memo: Vec<f64> = vec![f64::MAX; graph.len()];
        let result = dfs(graph.len() - 1, graph, &mut memo, mid) <= 0.0;
        if result {
            right = mid;
        } else {
            left = mid;
        }
        mid = (left + right) / 2.0;
    }

    mid
}

fn main() {
    let test_cases = read_line_as_numbers()[0];

    let mut output = BufWriter::new(stdout());
    for _ in 0..test_cases {
        let (n, r) = {
            let line = read_line_as_numbers();
            (line[0], line[1])
        };

        let (_start, _end) = {
            let line = read_line_as_numbers();
            (line[0] - 1, line[1] - 1)
        };

        let mut graph = vec![vec![]; n];
        let mut min_effort = f64::MAX;
        let mut max_effort = f64::MIN;

        for _ in 0..r {
            let (u, v, s, d) = {
                let line = read_line_as_numbers();
                (line[0] - 1, line[1] - 1, line[2], line[3])
            };

            let e = if s <= 60 {
                70 - s
            } else {
                10
            };
            let e = e as f64;
            let d = d as f64;
            min_effort = min_effort.min(e);
            max_effort = max_effort.max(e);

            graph[v].push((u, e, d));
        }

        writeln!(output, "{:.2}", binary_search(min_effort, max_effort, &graph)).unwrap();
    }
}
