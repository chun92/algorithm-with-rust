use std::io::{stdin, stdout, Write, BufWriter};

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn dfs(graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, v: usize, count: &mut usize, result: &mut Vec<usize>) {
    visited[v] = true;
    for i in 0..graph[v].len() {
        let val = graph[v][i];
        if !visited[val] {
            *count += 1;
            result[val] = *count;
            dfs(graph, visited, val, count, result);
        }
    }
}

fn main() {
    let (n, m, r) = {
        let v = read_line_as_numbers();
        (v[0], v[1], v[2])
    };

    let mut graph = vec![Vec::<usize>::new(); n];

    for _ in 0..m {
        let (a, b) = {
            let v = read_line_as_numbers();
            (v[0], v[1])
        };
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }


    graph.iter_mut().for_each(|v| v.sort_by(|a, b| b.cmp(a)));

    let mut count = 1;
    let mut visited = vec![false; n];
    let mut result = vec![0; n];
    result[r - 1] = count;
    dfs(&graph, &mut visited, r - 1, &mut count, &mut result);
    
    let mut out = BufWriter::new(stdout());
    for i in 0..n {
        writeln!(out, "{}", result[i]).unwrap();
    }
}
