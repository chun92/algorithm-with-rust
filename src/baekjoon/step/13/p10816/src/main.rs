use std::io::{stdin, stdout, Write, BufWriter};
use std::collections::HashMap;

fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let _n = read_line_as_numbers()[0];
    let mut map = HashMap::new();
    let samples = read_line_as_numbers();
    samples
        .iter()
        .for_each(|x| {
            let n = map.get(x).unwrap_or(&0);
            map.insert(x, n + 1);
        });
    let _m = read_line_as_numbers()[0];
    let results = read_line_as_numbers();
    let mut out = BufWriter::new(stdout());
    results
        .iter()
        .enumerate()
        .for_each(|(i, x)| {
            let n = map.get(x).unwrap_or(&0);
            write!(out, "{}", n).unwrap();
            if i != results.len() - 1 {
                write!(out, " ").unwrap();
            } else {
                writeln!(out).unwrap();
            }
        });
}
