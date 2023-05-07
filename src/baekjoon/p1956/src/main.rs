use std::io::stdin;

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

fn floid_warshall(graph: &mut Vec<Vec<Edge>>, cycle_check: &mut Vec<Edge>, v: usize) {
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
                        
                        if i == j {
                            if w1 + w2 != 0 {
                                if let Edge::WEIGHT(w3) = cycle_check[i] {
                                    cycle_check[i] = Edge::WEIGHT(std::cmp::min(w3, w1 + w2));
                                } else {
                                    cycle_check[i] = Edge::WEIGHT(w1 + w2);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let (n, m) = {
        let line = read_line_as_numbers();
        (line[0] as usize, line[1] as usize)
    };
    let mut graph = vec![vec![Edge::INFINITY; n]; n];
    let mut cycle_check = vec![Edge::INFINITY; n];

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

    floid_warshall(&mut graph, &mut cycle_check, n);
    let mut min = i64::max_value();
    for i in 0..n {
        if let Edge::WEIGHT(w) = cycle_check[i] {
            min = std::cmp::min(min, w);
        }
    }

    if min == i64::max_value() {
        println!("-1");
    } else {
        println!("{}", min);
    }
}
