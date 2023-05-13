use std::io;
use std::collections::VecDeque;
use std::cmp;

fn dfs(graph: &Vec<Vec<(usize, i32)>>, start: usize) -> (usize, i32) {
    let mut visited = vec![false; graph.len()];
    let mut distance = vec![0; graph.len()];
    let mut stack = VecDeque::new();
    stack.push_back(start);
    visited[start] = true;

    let mut max_distance = 0;
    let mut max_node = start;

    while let Some(node) = stack.pop_back() {
        for &(next_node, weight) in &graph[node] {
            if !visited[next_node] {
                visited[next_node] = true;
                distance[next_node] = distance[node] + weight;
                stack.push_back(next_node);
                if distance[next_node] > max_distance {
                    max_distance = distance[next_node];
                    max_node = next_node;
                }
            }
        }
    }

    (max_node, max_distance)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let v: usize = input.trim().parse().unwrap();

    let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; v + 1];

    for _ in 0..v {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let edge_info: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let u = edge_info[0] as usize;
        let mut i = 1;

        while edge_info[i] != -1 {
            let v = edge_info[i] as usize;
            let w = edge_info[i + 1];
            graph[u].push((v, w));
            graph[v].push((u, w));
            i += 2;
        }
    }

    let (node, _) = dfs(&graph, 1);
    let (_, diameter) = dfs(&graph, node);
    println!("{}", diameter);
}
