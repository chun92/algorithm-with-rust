use std::{io::{stdin, stdout, Write, BufWriter}, collections::VecDeque};

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (n, m, r) = {
        let v = read_line_as_numbers();
        (v[0], v[1], v[2])
    };

    let mut graph = vec![Vec::<usize>::new(); n];

    for _ in 0..m {
        let (a, b) = {
            let v = read_line_as_numbers();
            (v[0], v[1])
        };
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }


    graph.iter_mut().for_each(|v| v.sort());

    let mut count = 1;
    let mut visited = vec![false; n];
    let mut result = vec![0; n];
    let mut queue = VecDeque::<usize>::new();
    queue.push_back(r - 1);

    while let Some(v) = queue.pop_front() {
        if visited[v] {
            continue;
        }
        visited[v] = true;
        result[v] = count;
        count += 1;
        for &u in graph[v].iter() {
            if !visited[u] {
                queue.push_back(u);
            }
        }
    }
    
    let mut out = BufWriter::new(stdout());
    for i in 0..n {
        writeln!(out, "{}", result[i]).unwrap();
    }
}
