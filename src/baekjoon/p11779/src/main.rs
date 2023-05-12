use std::io::{stdin, stdout, Write, BufWriter};
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

fn read_line_as_numbers() -> Vec<usize> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect()
}

fn make_graph() -> (usize, HashMap<usize, Vec<(usize, usize)>>, usize, usize) {
    let mut graph = HashMap::new();

    let (v, e) = {
        (read_line_as_numbers()[0], read_line_as_numbers()[0])
    };

    for _ in 0..e {
        let input = read_line_as_numbers();
        let (u, v, w) = (input[0], input[1], input[2]);
        graph.entry(u).or_insert(Vec::new()).push((v, w));
    }

    let args = read_line_as_numbers();
    let start = args[0];
    let end = args[1];

    (v, graph, start, end)
}

fn dijkstra(v: usize, graph: HashMap<usize, Vec<(usize, usize)>>, start: usize) -> (Vec<usize>, Vec<usize>) {
    let mut distance = vec![usize::MAX; v + 1];
    let mut prev = vec![0; v + 1];
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
                prev[*next_node] = node;
                distance[*next_node] = next_cost;
                heap.push((Reverse(next_cost), *next_node));
            }
        }
    }

    (distance, prev)
}

fn print_path(distance: &Vec<usize>, prev: &Vec<usize>, start: usize, end: usize) {
    let mut path = Vec::new();
    let mut node = end;

    while node != start {
        path.push(node);
        node = prev[node];
    }

    path.push(start);
    path.reverse();

    let mut out = BufWriter::new(stdout());
    writeln!(out, "{}", distance[end]).unwrap();
    writeln!(out, "{}", path.len()).unwrap();
    writeln!(out, "{}", path.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")).unwrap();
}

fn main() {
    let (v, graph, start, end) = make_graph();
    let (distance, prev) = dijkstra(v, graph, start);
    print_path(&distance, &prev, start, end);
}
