use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers:Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    
    let mut iter = numbers.iter();
    let first = *iter.next().unwrap() as f64;
    let second = *iter.next().unwrap() as f64;
    let result = first / second;
    println!("{}", result);
}
