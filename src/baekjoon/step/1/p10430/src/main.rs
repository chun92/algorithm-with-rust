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
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    let c = iter.next().unwrap();

    println!("{}", (a + b) % c);
    println!("{}", ((a % c) + (b % c)) % c);
    println!("{}", (a * b) % c);
    println!("{}", ((a % c) * (b % c)) % c);
}