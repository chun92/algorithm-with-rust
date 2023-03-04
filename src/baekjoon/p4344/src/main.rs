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

fn print_percentage_over_average(vec: Vec<i32>) {
    let n = vec[0] as usize;
    let slice:Vec<f64> = vec[1..=n]
        .iter()
        .map(|x| *x as f64)
        .collect();

    let sum:f64 = slice
        .iter()
        .sum();

    let average = sum / (n as f64);

    let filter:Vec<f64> = slice
        .into_iter()
        .filter(|x| *x > average)
        .collect();

    let result = filter.len() as f64 / n as f64 * 100.0;
    println!("{:.3}%", (result * 1000.0).round()/1000.0);
}

fn main() {
    let count = read_line_as_number();
    for _ in 0..count {
        let elem = read_line_as_numbers();
        print_percentage_over_average(elem);
    }
}