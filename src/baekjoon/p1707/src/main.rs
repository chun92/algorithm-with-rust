fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn check_bipartite_graph(graph: &Vec<Vec<usize>>) -> bool {
    let n = graph.len();
    let mut color = vec![0; n];
    let mut queue = std::collections::VecDeque::new();
    for i in 0..n {
        if color[i] != 0 {
            continue;
        }
        queue.push_back(i);
        color[i] = 1;
        while !queue.is_empty() {
            let u = queue.pop_front().unwrap();
            for &v in &graph[u] {
                if color[v] == 0 {
                    color[v] = -color[u];
                    queue.push_back(v);
                } else if color[v] == color[u] {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    let test_cases = read_line_as_numbers()[0];
    for _ in 0..test_cases {
        let numbers = read_line_as_numbers();
        let (v, e) = (numbers[0], numbers[1]);
        let mut graph = vec![vec![]; v];
        for _ in 0..e {
            let numbers = read_line_as_numbers();
            let (u, v) = (numbers[0] - 1, numbers[1] - 1);
            graph[u].push(v);
            graph[v].push(u);
        }
        println!("{}", if check_bipartite_graph(&graph) { "YES" } else { "NO" });
    }
}
