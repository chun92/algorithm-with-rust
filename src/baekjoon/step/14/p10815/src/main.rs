use std::collections::HashSet;
use std::io;
use io::Write;

fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let _n = read_line_as_numbers()[0];
    let mut set = HashSet::new();

    let numbers = read_line_as_numbers();
    numbers.iter().for_each(|x| {
        set.insert(x);
    });

    let _m = read_line_as_numbers()[0];
    let results = read_line_as_numbers();
    let mut out = io::BufWriter::new(io::stdout());
    results.iter().enumerate().for_each(|(i, x)| {
        if set.contains(x) {
            write!(out, "1").unwrap();
        } else {
            write!(out, "0").unwrap();
        }

        if i != results.len() - 1 {
            write!(out, " ").unwrap();
        } else {
            writeln!(out).unwrap();
        }
    });
}
