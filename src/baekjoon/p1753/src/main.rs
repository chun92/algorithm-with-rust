use std::io::{stdin, stdout, Write, BufWriter};
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

fn read_line_as_numbers() -> Vec<usize> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect()
}

fn make_graph() -> (usize, usize, HashMap<usize, Vec<(usize, usize)>>) {
    let mut graph = HashMap::new();

    let (v, e) = {
        let input = read_line_as_numbers();
        (input[0], input[1])
    };

    let start = read_line_as_numbers()[0];

    for _ in 0..e {
        let input = read_line_as_numbers();
        let (u, v, w) = (input[0], input[1], input[2]);
        graph.entry(u).or_insert(Vec::new()).push((v, w));
    }

    (v, start, graph)
}

fn dijkstra(v: usize, start: usize, graph: HashMap<usize, Vec<(usize, usize)>>) -> Vec<usize> {
    let mut distance = vec![usize::MAX; v + 1];
    let mut heap = BinaryHeap::new();

    distance[start] = 0;
    heap.push((Reverse(0), start));

    while let Some((Reverse(cost), node)) = heap.pop() {
        if distance[node] < cost {
            continue;
        }
        
        for (next_node, next_cost) in graph.get(&node).unwrap_or(&Vec::new()) {
            let next_cost = cost + next_cost;

            if next_cost < distance[*next_node] {
                distance[*next_node] = next_cost;
                heap.push((Reverse(next_cost), *next_node));
            }
        }
    }

    distance
}

fn print_distances(distance: Vec<usize>) {
    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    for i in 1..distance.len() {
        if distance[i] == usize::MAX {
            writeln!(out, "INF").unwrap();
        } else {
            writeln!(out, "{}", distance[i]).unwrap();
        }
    }
}

fn main() {
    let (v, start, graph) = make_graph();
    let distance = dijkstra(v, start, graph);
    print_distances(distance);
}
