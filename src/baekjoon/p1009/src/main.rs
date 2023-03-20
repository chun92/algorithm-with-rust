use std::io::{ BufWriter, Write, stdout };
fn read_line_as_numbers() -> Vec<u64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_first_digit(a: u64, b: u64) -> u64 {
    let mut result = 1;
    for _ in 0..b {
        result *= a;
        result %= 10;
    }
    if result == 0 {
        result = 10;
    }
    result
}

fn main() {
    let n = read_line_as_numbers()[0];
    let mut out = BufWriter::new(stdout());
    for _ in 0..n {
        let numbers = read_line_as_numbers();
        writeln!(out, "{}", get_first_digit(numbers[0], numbers[1])).unwrap();
    }
}
