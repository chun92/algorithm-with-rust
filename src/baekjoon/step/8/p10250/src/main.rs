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

fn read_line_as_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let count = read_line_as_number();
    for _ in 0..count {
        let numbers = read_line_as_numbers();
        let (h, _w, n) = (numbers[0], numbers[1], numbers[2]);
        let a = {
            if n % h == 0 {
                h
            } else {
                n % h
            }
        };
        let b = {
            if n % h == 0 {
                n / h
            } else {
                n / h + 1
            }
        };
        let result = a * 100 + b;
        println!("{}", result);
    }
}
