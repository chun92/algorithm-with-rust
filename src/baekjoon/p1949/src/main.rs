fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn dp(
    graph: &Vec<Vec<usize>>,
    weigths: &Vec<usize>,
    node: usize,
    parent: usize,
) -> (usize, usize) {
    let mut res = (0, 0);
    for &child in graph[node].iter() {
        if child == parent {
            continue;
        }
        let child_dp = dp(graph, weigths, child, node);
        res.0 += child_dp.1;
        res.1 += std::cmp::max(child_dp.0, child_dp.1);
    }
    res.0 += weigths[node];
    res
}

fn main() {
    let n = read_line_as_numbers()[0];
    let weigths = read_line_as_numbers();
    let mut graph = vec![vec![]; n];

    for _ in 0..n - 1 {
        let edge = read_line_as_numbers();
        graph[edge[0] - 1].push(edge[1] - 1);
        graph[edge[1] - 1].push(edge[0] - 1);
    }
    let res = dp(&graph, &weigths, 0, 0);
    println!("{}", std::cmp::max(res.0, res.1));
}
