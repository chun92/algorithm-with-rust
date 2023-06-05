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

fn dijkstra(edges: Vec<Vec<Edge>>, start: usize) -> Vec<usize> {
    let mut dist = vec![std::usize::MAX; edges.len()];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(Edge { to: start, cost: 0 });

    while let Some(Edge { to, cost }) = heap.pop() {
        if cost > dist[to] {
            continue;
        }
        for edge in &edges[to] {
            let next = Edge {
                to: edge.to,
                cost: cost + edge.cost,
            };
            if next.cost < dist[next.to] {
                dist[next.to] = next.cost;
                heap.push(next);
            }
        }
    }

    dist
}

fn main() {
    let (n, m, x) = {
        let v = read_line_as_numbers();
        (v[0], v[1], v[2] - 1)
    };

    let mut edges = vec![vec![]; n];
    let mut reverse_edges = vec![vec![]; n];

    for _ in 0..m {
        let (src, dst, cost) = {
            let v = read_line_as_numbers();
            (v[0] - 1, v[1] - 1, v[2])
        };
        edges[src].push(Edge { to: dst, cost });
        reverse_edges[dst].push(Edge { to: src, cost });
    }

    let dist_from_x = dijkstra(edges, x);
    let dist_to_x = dijkstra(reverse_edges, x);

    let mut max_dist = 0;
    for i in 0..n {
        let dist = dist_from_x[i] + dist_to_x[i];
        if dist > max_dist {
            max_dist = dist;
        }
    }
    println!("{}", max_dist);
}
