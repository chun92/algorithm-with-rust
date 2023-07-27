fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut graph = Vec::new();
    for _ in 0..n {
        graph.push(read_line_as_numbers());
    }

    let mut memo = vec![vec![0; m]; n];
    memo[0][0] = graph[0][0];
    for i in 0..n {
        for j in 0..m {
            let cur = memo[i][j];
            if i + 1 < n {
                memo[i + 1][j] = memo[i + 1][j].max(cur + graph[i + 1][j]);
            }

            if j + 1 < m {
                memo[i][j + 1] = memo[i][j + 1].max(cur + graph[i][j + 1]);
            }

            if i + 1 < n && j + 1 < m {
                memo[i + 1][j + 1] = memo[i + 1][j + 1].max(cur + graph[i + 1][j + 1]);
            }
        }
    }

    println!("{}", memo[n - 1][m - 1]);
}
