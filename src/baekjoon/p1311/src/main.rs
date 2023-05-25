fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn dp(n: usize, path: usize, graph: &Vec<Vec<usize>>, memo: &mut Vec<usize>) -> usize {
    if memo[path] != 0 {
        return memo[path];
    }

    let mut min = std::usize::MAX;
    for i in 0..graph.len() {
        if path & (1 << i) == 0 {
            continue;
        }
        let cost = 
            if n == 0 {
                graph[0][i]
            } else {
                dp(n - 1, path ^ (1 << i), graph, memo) + graph[n][i]
            };
        if cost < min {
            min = cost;
        }
    }

    memo[path] = min;
    min
}

fn main() {
    let n = read_line_as_numbers()[0];

    let mut memo = vec![0; 1 << n];
    let mut graph = Vec::new();
    for _ in 0..n {
        graph.push(read_line_as_numbers());
    }

    println!("{}", dp(n - 1, (1 << n) - 1, &graph, &mut memo));
}
