use std::io;

fn read_line_as_number() -> i32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let value = s.trim().parse::<i32>().unwrap();
    value
}

fn read_line_as_numbers() -> Vec<i32> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    
    let values:Vec<i32> = s
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    values
}

fn main() {
    let n = read_line_as_number() as f64;
    let numbers = read_line_as_numbers();
    let max = *numbers.iter().max().unwrap() as f64;
    let sum = numbers.iter().fold(0 as f64, |total, x| {
        total + (*x as f64) / max * 100.0
    });
    println!("{}", sum / n);
}
