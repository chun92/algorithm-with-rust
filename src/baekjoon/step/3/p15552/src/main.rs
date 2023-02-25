use std::io;
use io::Write;
fn read_line_as_number() -> i32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let value = s.trim().parse::<i32>().unwrap();
    return value;
}

fn read_line_as_numbers() -> Vec<i32> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    
    let values:Vec<i32> = s
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    return values;
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let count = read_line_as_number();
    for _ in 0..count {
        let numbers = read_line_as_numbers();
        let result = numbers.into_iter().sum::<i32>();
        writeln!(out, "{}", result).unwrap();
    }
}