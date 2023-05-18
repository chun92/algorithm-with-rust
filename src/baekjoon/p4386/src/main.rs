use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, PartialEq)]
struct Edge {
    cost: f64,
    node: usize,
}

impl Eq for Edge {}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    (dx * dx + dy * dy).sqrt()
}

fn prim(points: &Vec<(f64, f64)>) -> f64 {
    let mut total_cost = 0.0;
    let mut visited = vec![false; points.len()];
    let mut heap = BinaryHeap::new();

    let mut current_point;
    heap.push(Edge { cost: 0.0, node: 0 });

    while let Some(Edge { cost, node }) = heap.pop() {
        if !visited[node] {
            total_cost += cost;
            visited[node] = true;
            current_point = points[node];
            for (other_node, other_point) in points.iter().enumerate() {
                if !visited[other_node] {
                    heap.push(Edge {
                        cost: get_distance(current_point, *other_point),
                        node: other_node,
                    });
                }
            }
        }
    }

    total_cost
}

fn read_line_as_number() -> usize {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

fn read_line_as_floats() -> Vec<f64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_number();

    let mut points = Vec::new();
    for _ in 0..n {
        let floats = read_line_as_floats();
        points.push((floats[0], floats[1]));
    }
    let result = prim(&points);
    println!("{:.2}", result);
}