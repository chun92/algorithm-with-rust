use std::io::{Write, BufWriter, stdout};
use std::collections::HashSet;

fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let _n = read_line_as_numbers()[0];
    let numbers = read_line_as_numbers();
    let _m = read_line_as_numbers()[0];
    let targets = read_line_as_numbers();
    
    let numbers: HashSet<i32> = numbers.into_iter().collect();
    let mut out = BufWriter::new(stdout());

    targets
        .iter()
        .for_each(|x| {
            if numbers.contains(x) {
                writeln!(out, "1").unwrap();
            } else {
                writeln!(out, "0").unwrap();
            }
        })
}
