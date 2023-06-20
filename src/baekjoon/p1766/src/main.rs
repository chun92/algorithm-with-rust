use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut graph = vec![vec![]; n + 1];
    let mut parent = vec![0; n + 1];
    for _ in 0..m {
        let (src, dst) = {
            let v = read_line_as_numbers();
            (v[0], v[1])
        };
        graph[src].push(dst);
        parent[dst] += 1;
    }
    
    let mut heap = BinaryHeap::new();

    for i in 1..=n {
        if parent[i] == 0 {
            heap.push(Reverse(i));
        }
    }

    while let Some(Reverse(node)) = heap.pop() {
        print!("{} ", node);
        for &next in &graph[node] {
            parent[next] -= 1;
            if parent[next] == 0 {
                heap.push(Reverse(next));
            }
        }
    }
}
