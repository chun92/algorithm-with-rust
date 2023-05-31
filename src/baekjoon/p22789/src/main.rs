use std::io::{stdin, stdout, BufWriter, Write};

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

const MAX: f64 = 1e9;
const EPS: f64 = 1e-9;

fn solve(start: usize, end: usize, graph: &Vec<Vec<(usize, usize, usize)>>) -> f64 {
    let n = graph.len();
    let mut l = 0.0;
    let mut r = MAX;
    let mut f = vec![0.0; n];

    while r - l > EPS {
        let mid = (l + r) / 2.0;
        for i in 0..n {
            f[i] = if i == start { 0.0 } else { MAX };
            for &(j, d, e) in &graph[i] {
                f[i] = f[i].min(f[j] + d as f64 * (e as f64 - mid));
            }
        }
        if f[end] <= 0.0 {
            r = mid;
        } else {
            l = mid;
        }
    }

    l
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
        }

        writeln!(output, "{:.2}", solve(start, end, &graph)).unwrap();
    }
}
