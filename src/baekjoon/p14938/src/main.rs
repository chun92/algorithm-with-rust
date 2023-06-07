use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq)]
struct Edge {
    to: usize,
    cost: usize,
}

impl Ord for Edge {
    fn cmp(&self, other: &Edge) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}


fn dijkstra(n: usize, edges: &Vec<Vec<Edge>>, start: usize) -> Vec<usize> {
    let mut dist = vec![std::usize::MAX; n];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(Edge { to: start, cost: 0 });

    while let Some(Edge { to, cost }) = heap.pop() {
        if dist[to] < cost {
            continue;
        }

        for edge in &edges[to] {
            let next = Edge { to: edge.to, cost: cost + edge.cost };
            if next.cost < dist[next.to] {
                dist[next.to] = next.cost;
                heap.push(next);
            }
        }
    }
    dist
}

fn main() {
    let (n, m, r) = {
        let v = read_line_as_numbers();
        (v[0], v[1], v[2])
    };

    let mut vertices = Vec::new();
    let v = read_line_as_numbers();
    for i in 0..n {
        vertices.push(v[i]);
    }

    let mut edges = vec![Vec::new(); n];
    for _ in 0..r {
        let (u, v, l) = {
            let v = read_line_as_numbers();
            (v[0] - 1, v[1] - 1, v[2])
        };
        let edge = Edge { to: v, cost: l };
        edges[u].push(edge);
        let edge = Edge { to: u, cost: l };
        edges[v].push(edge);
    }

    let mut max = 0;
    for i in 0..n {
        let dist = dijkstra(n, &edges, i);
        let mut sum = 0;
        for j in 0..n {
            if dist[j] <= m {
                sum += vertices[j];
            }
        }
        if sum > max {
            max = sum;
        }
    }

    println!("{}", max);
}
