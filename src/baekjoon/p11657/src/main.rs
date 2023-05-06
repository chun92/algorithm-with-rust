use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Distance {
    Infinite,
    Finite(i64),
}

fn read_line_as_numbers() -> Vec<i64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn belman_ford(graph: &Vec<Vec<(usize, i64)>>, start: usize, v: usize) -> Result<Vec<i64>, Box<dyn Error>> {
    let mut dist = vec![Distance::Infinite; v + 1];
    dist[start] = Distance::Finite(0);

    for _ in 0..v - 1 {
        for (u, edges) in graph.iter().enumerate() {
            if let Distance::Finite(d) = dist[u] {
                for (v, w) in edges {
                    if let Distance::Finite(d2) = dist[*v] {
                        if d2 > d + *w {
                            dist[*v] = Distance::Finite(d + *w);
                        }
                    } else {
                        dist[*v] = Distance::Finite(d + *w);
                    }
                }
            }
        }
    }

    for (u, edges) in graph.iter().enumerate() {
        if let Distance::Finite(d) = dist[u] {
            for (v, w) in edges {
                if let Distance::Finite(d2) = dist[*v] {
                    if d2 > d + *w {
                        return Err("Negative Cycle".into());
                    }
                }
            }
        }
    }

    Ok(dist.iter().map(|d| match d {
        Distance::Infinite => std::i64::MAX,
        Distance::Finite(d) => *d,
    }).collect())
}

fn main() {
    let (v, e) = {
        let line = read_line_as_numbers();
        (line[0], line[1])
    };

    let mut graph = vec![Vec::new(); (v + 1) as usize];
    for _ in 0..e {
        let line = read_line_as_numbers();
        let (a, b, c) = (line[0], line[1], line[2]);
        graph[a as usize].push((b as usize, c));
    }

    let result = belman_ford(&graph, 1, v as usize);
    match result {
        Ok(dist) => {
            for i in 2..=v as usize {
                if dist[i] == std::i64::MAX {
                    println!("-1");
                } else {
                    println!("{}", dist[i]);
                }
            }
        }
        Err(_) => {
            println!("-1");
        }
    }
}
