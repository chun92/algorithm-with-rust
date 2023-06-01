use std::collections::VecDeque;
use std::collections::HashMap;
use std::io::{stdin, stdout, BufWriter, Write};

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn solve(start: usize, end: usize, graph: &Vec<Vec<(usize, usize, usize)>>, enters: &mut Vec<usize>) -> f64 {
    let mut memo: Vec<HashMap<usize, usize>> = vec![HashMap::new(); graph.len()];

    let mut queue = VecDeque::new();
    queue.push_back(start);
    memo[start].insert(0, 0);

    while let Some(idx) = queue.pop_front() {
        for (v, e, d) in &graph[idx] {
            enters[*v] -= 1;
            if enters[*v] == 0 {
                queue.push_back(*v);
            }

            let costs_with_distances = memo[idx].clone();

            for (&distance, &cost) in &costs_with_distances {
                let new_cost = cost + e;
                let new_distance = distance + d;

                let costs_with_distances = memo[*v].entry(new_distance).or_insert(new_cost);
                *costs_with_distances = (*costs_with_distances).min(new_cost);
            }
        }
    }

    let mut result = f64::MAX;
    for (d, e) in memo[end].iter() {
        result = result.min((*e as f64) / (*d as f64));
    }

    result
}

fn main() {
    let test_cases = read_line_as_numbers()[0];

    let mut output = BufWriter::new(stdout());
    for _ in 0..test_cases {
        let (n, r) = {
            let line = read_line_as_numbers();
            (line[0], line[1])
        };

        let (start, end) = {
            let line = read_line_as_numbers();
            (line[0] - 1, line[1] - 1)
        };

        let mut graph = vec![vec![]; n];
        let mut enters = vec![0; n];

        for _ in 0..r {
            let (u, v, s, d) = {
                let line = read_line_as_numbers();
                (line[0] - 1, line[1] - 1, line[2], line[3])
            };

            let e = if s <= 60 {
                d * (70 - s)
            } else {
                d * (s - 50)
            };

            graph[u].push((v, e, d));
            enters[v] += 1;
        }

        writeln!(output, "{:.2}", solve(start, end, &graph, &mut enters)).unwrap();
    }
}
