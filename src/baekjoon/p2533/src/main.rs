fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn dp(graph: &Vec<Vec<usize>>, v: usize, p: usize) -> (usize, usize) {
    let mut res1 = 1;
    let mut res2 = 0;
    for &u in graph[v].iter() {
        if u == p {
            continue;
        }

        let (a, b) = dp(graph, u, v);
        res1 += std::cmp::min(a, b);
        res2 += a;
    }

    (res1, res2)
}

fn main() {
    let n = read_line_as_numbers()[0];
    let mut graph = vec![vec![]; n];

    for _ in 0..n - 1 {
        let line = read_line_as_numbers();
        let u = line[0] - 1;
        let k = line[1] - 1;
        graph[u].push(k);
        graph[k].push(u);
    }

    let (a, b) = dp(&graph, 0, 0);
    println!("{}", std::cmp::min(a, b));
}
