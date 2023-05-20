use std::io::{stdin, stdout, Write, BufWriter};

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn make_tree(tree: &mut Vec<Vec<usize>>, graph: &Vec<Vec<usize>>, parent: usize, visited: &mut Vec<bool>) {
    visited[parent] = true;
    for &v in &graph[parent] {
        if !visited[v] {
            tree[parent].push(v);
            make_tree(tree, graph, v, visited);
        }
    }
}

fn dp(tree: &Vec<Vec<usize>>, memo: &mut Vec<usize>, parent: usize) -> usize {
    if memo[parent] != 0 {
        return memo[parent];
    }

    let mut sum = 1;
    for &v in &tree[parent] {
        sum += dp(tree, memo, v);
    }
    memo[parent] = sum;
    sum
}

fn main() {
    let (n, r, q) = {
        let v = read_line_as_numbers();
        (v[0], v[1], v[2])
    };
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        let (u, v) = {
            let v = read_line_as_numbers();
            (v[0] - 1, v[1] - 1)
        };
        g[u].push(v);
        g[v].push(u);
    }

    let mut tree = vec![vec![]; n];
    let mut visited = vec![false; n];
    make_tree(&mut tree, &g, r - 1, &mut visited);

    let mut memo = vec![0; n];
    let mut output = BufWriter::new(stdout());
    for _ in 0..q {
        let u = read_line_as_numbers()[0] - 1;
        writeln!(output, "{}", dp(&tree, &mut memo, u)).unwrap();
    }
}
