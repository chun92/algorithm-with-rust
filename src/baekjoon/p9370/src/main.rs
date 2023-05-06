use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}


fn dijkstra(graph: &HashMap<usize, Vec<(usize, usize)>>, start: usize, v: usize) -> Vec<usize> {
    let mut dist = vec![usize::MAX; v + 1];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), start));
    while let Some((Reverse(d), v)) = heap.pop() {
        if dist[v] <= d {
            continue;
        }

        dist[v] = d;

        if let Some(edges) = graph.get(&v) {
            for &(u, w) in edges {
                heap.push((Reverse(d + u), w));
            }
        }
    }
    dist
}

fn main() {
    let test_cases = read_line_as_numbers()[0];
    for _ in 0..test_cases {
        let (n, m, t) = {
            let v = read_line_as_numbers();
            (v[0], v[1], v[2])
        };

        let (s, g, h) = {
            let v = read_line_as_numbers();
            (v[0], v[1], v[2])
        };

        let mut graph = HashMap::new();
        let mut mid = 0;
        for _ in 0..m {
            let (a, b, d) = {
                let v = read_line_as_numbers();
                (v[0], v[1], v[2])
            };
            graph.entry(a).or_insert_with(Vec::new).push((d, b));
            graph.entry(b).or_insert_with(Vec::new).push((d, a));

            if (a == g && b == h) || (a == h && b == g) {
                mid = d;
            }
        }


        let dist_src = dijkstra(&graph, s, n);
        let dist_src_g = dist_src[g];
        let dist_src_h = dist_src[h];

        let mut result = Vec::new();
        for _ in 0..t {
            let d = read_line_as_numbers()[0];

            let dist_dst = dijkstra(&graph, d, n);
            let dist_dst_g = dist_dst[g];
            let dist_dst_h = dist_dst[h];
            let dist = dist_src[d];

            if dist == dist_src_g + mid + dist_dst_h || dist == dist_src_h + mid + dist_dst_g {
                result.push(d);
            }
        }
        result.sort_unstable();

        let result = result.iter()
            .map(|&v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", result);
    }
}
