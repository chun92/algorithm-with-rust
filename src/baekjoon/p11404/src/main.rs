use std::io::{stdin, stdout, BufWriter, Write};

fn read_line_as_numbers() -> Vec<i64> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Edge {
    INFINITY,
    WEIGHT(i64),
}

fn floid_warshall(graph: &mut Vec<Vec<Edge>>, v: usize) {
    for k in 0..v {
        for i in 0..v {
            for j in 0..v {
                if let Edge::WEIGHT(w1) = graph[i][k] {
                    if let Edge::WEIGHT(w2) = graph[k][j] {
                        if let Edge::WEIGHT(w3) = graph[i][j] {
                            graph[i][j] = Edge::WEIGHT(std::cmp::min(w3, w1 + w2));
                        } else {
                            graph[i][j] = Edge::WEIGHT(w1 + w2);
                        }
                    }
                }
            }
        }
    }
}

fn print_graph(graph: &Vec<Vec<Edge>>, v: usize) {
    let mut stdout = BufWriter::new(stdout());
    for i in 0..v {
        for j in 0..v {
            match graph[i][j] {
                Edge::INFINITY => write!(stdout, "0").unwrap(),
                Edge::WEIGHT(w) => write!(stdout, "{}", w).unwrap(),
            }
            if j != v - 1 {
                write!(stdout, " ").unwrap();
            }
        }
        writeln!(stdout).unwrap();
    }
}

fn main() {
    let n = read_line_as_numbers()[0] as usize;
    let m = read_line_as_numbers()[0];
    let mut graph = vec![vec![Edge::INFINITY; n]; n];

    for _ in 0..m {
        let line = read_line_as_numbers();
        let (i, j, k) = (line[0] as usize - 1, line[1] as usize - 1, line[2]);
        if let Edge::WEIGHT(w) = graph[i][j] {
            graph[i][j] = Edge::WEIGHT(std::cmp::min(w, k));
        } else {
            graph[i][j] = Edge::WEIGHT(k);
        }
    }
    
    for i in 0..n {
        graph[i][i] = Edge::WEIGHT(0);
    }

    floid_warshall(&mut graph, n);
    print_graph(&graph, n);
}
