fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn has_negative_cycle(graph: &Vec<Vec<(usize, i32)>>) -> bool {
    let n = graph.len();
    let mut dist = vec![2000000000; n];
    dist[1] = 0;

    for _ in 0..n - 1 {
        let mut updated = false;
        for u in 1..n {
            for &(v, w) in &graph[u] {
                if dist[v] > dist[u] + w {
                    dist[v] = dist[u] + w;
                    updated = true;
                }
            }
        }
        if !updated {
            break;
        }
    }

    for u in 1..n {
        for &(v, w) in &graph[u] {
            if dist[v] > dist[u] + w {
                return true;
            }
        }
    }

    false
}

fn main() {
    let test_cases = read_line_as_numbers()[0];

    for _ in 0..test_cases {
        let (n, m, w) = {
            let v = read_line_as_numbers();
            (v[0], v[1], v[2])
        };

        let mut graph = vec![vec![]; n as usize + 1];

        for _ in 0..m {
            let (s, e, t) = {
                let v = read_line_as_numbers();
                (v[0] as usize, v[1] as usize, v[2])
            };

            graph[s].push((e, t));
            graph[e].push((s, t));
        }

        for _ in 0..w {
            let (s, e, t) = {
                let v = read_line_as_numbers();
                (v[0] as usize, v[1] as usize, v[2])
            };

            graph[s].push((e, -t));
        }

        println!("{}", if has_negative_cycle(&graph) { "YES" } else { "NO" });
    }
}