use std::io::stdin;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

fn read_line_as_numbers() -> Vec<usize> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect()
}

fn make_graph() -> (usize, HashMap<usize, Vec<(usize, usize)>>) {
    let mut graph = HashMap::new();

    let (v, e) = {
        let input = read_line_as_numbers();
        (input[0], input[1])
    };

    for _ in 0..e {
        let input = read_line_as_numbers();
        let (u, v, w) = (input[0], input[1], input[2]);
        graph.entry(u).or_insert(Vec::new()).push((v, w));
        graph.entry(v).or_insert(Vec::new()).push((u, w));
    }

    (v, graph)
}

fn dijkstra(v: usize, start: usize, graph: &HashMap<usize, Vec<(usize, usize)>>) -> Vec<u64> {
    let mut distance = vec![u64::MAX; v + 1];
    let mut heap = BinaryHeap::new();

    distance[start] = 0;
    heap.push((Reverse(0), start));

    while let Some((Reverse(cost), node)) = heap.pop() {
        if distance[node] < cost {
            continue;
        }
        
        for (next_node, next_cost) in graph.get(&node).unwrap_or(&Vec::new()) {
            let next_cost = cost as u64 + *next_cost as u64;

            if next_cost < distance[*next_node] {
                distance[*next_node] = next_cost;
                heap.push((Reverse(next_cost), *next_node));
            }
        }
    }

    distance
}
fn main() {
    let (v, graph) = make_graph();
    let start = 1;
    let (inter1, inter2) = {
        let input = read_line_as_numbers();
        (input[0], input[1])
    };
    let distance_from_src = dijkstra(v, start, &graph);
    let distance_from_dst = dijkstra(v, v, &graph);
    let distance_inter1 = dijkstra(v, inter1, &graph);

    let test1 = distance_from_src[inter1] == u64::MAX || distance_inter1[inter2] == u64::MAX || distance_from_dst[inter2] == u64::MAX;
    let test2 = distance_from_src[inter2] == u64::MAX || distance_inter1[inter2] == u64::MAX || distance_from_dst[inter1] == u64::MAX;

    match (test1, test2) {
        (true, true) => println!("-1"),
        (true, false) => println!("{}", distance_from_src[inter2] + distance_inter1[inter2] + distance_from_dst[inter1]),
        (false, true) => println!("{}", distance_from_src[inter1] + distance_inter1[inter2] + distance_from_dst[inter2]),
        (false, false) => {
            let test1 = distance_from_src[inter1] + distance_inter1[inter2] + distance_from_dst[inter2];
            let test2 = distance_from_src[inter2] + distance_inter1[inter2] + distance_from_dst[inter1];
            if test1 < test2 {
                println!("{}", test1);
            } else {
                println!("{}", test2);
            }
        }
    };
}
