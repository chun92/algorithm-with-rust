use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq)]
struct Node {
    index: usize,
    distance: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
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

fn dikjstra(graph: Vec<Vec<Node>>, start: usize, end: usize) -> usize {
    let mut heap = BinaryHeap::new();
    let mut distances = vec![usize::max_value(); graph.len()];
    distances[start] = 0;
    heap.push(Node { index: start, distance: 0 });

    while let Some(Node { index, distance }) = heap.pop() {
        if index == end {
            return distance;
        }

        if distance > distances[index] {
            continue;
        }

        for node in &graph[index] {
            let next = Node { index: node.index, distance: distance + node.distance };
            let next_index = next.index;
            let next_distance = next.distance;
            if next_distance < distances[next_index] {
                heap.push(next);
                distances[next_index] = next_distance;
            }
        }
    }

    distances[end]
}

fn main() {
    let n = read_line_as_numbers()[0];
    let m = read_line_as_numbers()[0];

    let mut graph = vec![Vec::new(); n];
    
    for _ in 0..m {
        let (src, dst, weight) = {
            let numbers = read_line_as_numbers();
            (numbers[0] - 1, numbers[1] - 1, numbers[2])
        };
        graph[src].push(Node { index: dst, distance: weight });
    }

    let (start, end) = {
        let numbers = read_line_as_numbers();
        (numbers[0], numbers[1])
    };

    println!("{}", dikjstra(graph, start - 1, end - 1));
}
