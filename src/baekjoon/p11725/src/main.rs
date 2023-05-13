use std::io::{stdin, stdout, BufWriter, Write};

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_numbers()[0];
    let mut graph = vec![Vec::new(); n];

    for _ in 0..n - 1 {
        let line = read_line_as_numbers();
        let u = line[0] - 1;
        let k = line[1] - 1;
        graph[u].push(k);
        graph[k].push(u);
    }

    let mut result = vec![0; n];

    fn dfs(graph: &Vec<Vec<usize>>, result: &mut Vec<usize>, v: usize, p: usize) {
        for &u in &graph[v] {
            if u == p {
                continue;
            }
            result[u] = v + 1;
            dfs(graph, result, u, v);
        }
    }

    dfs(&graph, &mut result, 0, 0);

    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    for i in 1..n {
        writeln!(out, "{}", result[i]).unwrap();
    }
}
