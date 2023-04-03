use std::io::{stdin, stdout, BufWriter, Write};
use std::collections::HashMap;

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn read_line_as_string() -> String {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let (n, m) = {
        let i = read_line_as_numbers();
        (i[0], i[1])
    };

    let mut map = HashMap::new();
    for _ in 0..n {
        let str = read_line_as_string();
        if str.len() >= m {
            let n = map.entry(str).or_insert(0);
            *n += 1;
        }
    }

    let mut v = map.into_iter().collect::<Vec<_>>();
    v.sort_by(|a, b| a.0.cmp(&b.0));
    v.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
    v.sort_by(|a, b| b.1.cmp(&a.1));

    let mut out = BufWriter::new(stdout());
    for (k, _) in v {
        writeln!(out, "{}", k).unwrap();
    }
}
