use std::io;

fn read_line_as_numbers() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let numbers = read_line_as_numbers();
    let (a, b, v) = (numbers[0], numbers[1], numbers[2]);
    let count = (v - b) as f64 / (a - b) as f64;
    println!("{}", count.ceil() as i32);
}
