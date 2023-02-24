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
    let first = iter.next().unwrap();
    let second = iter.next().unwrap();
    let res = first - second;
    println!("{}", res);
}