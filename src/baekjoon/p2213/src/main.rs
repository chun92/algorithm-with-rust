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
    memo: &mut Vec<Vec<usize>>,
    node: usize,
    parent: usize,
) -> (usize, usize) {
    if memo[node][0] != 0 {
        return (memo[node][0], memo[node][1]);
    }

    let mut res = (0, 0);
    for &child in graph[node].iter() {
        if child == parent {
            continue;
        }
        let child_dp = dp(graph, weigths, memo, child, node);
        res.0 += child_dp.1;
        res.1 += std::cmp::max(child_dp.0, child_dp.1);
    }
    res.0 += weigths[node];
    memo[node][0] = res.0;
    memo[node][1] = res.1;
    res
}

fn recover_path(memo: &Vec<Vec<usize>>, graph: &Vec<Vec<usize>>, node: usize, parent: usize, pickable: bool) -> Vec<usize> {
    let mut res = vec![];

    if pickable && memo[node][0] > memo[node][1] {
        res.push(node);
        for &child in graph[node].iter() {
            if child == parent {
                continue;
            }
            res.append(&mut recover_path(memo, graph, child, node, false));
        }
    } else {
        for &child in graph[node].iter() {
            if child == parent {
                continue;
            }
            res.append(&mut recover_path(memo, graph, child, node, true));
        }
    }

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

    let mut memo = vec![vec![0; 2]; n];

    let res = dp(&graph, &weigths, &mut memo, 0, 0);
    println!("{}", std::cmp::max(res.0, res.1));

    let mut path = recover_path(&memo, &graph, 0, 0, true);
    path.sort_unstable();
    let str = path
        .iter()
        .map(|&x| (x + 1).to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", str);
}
