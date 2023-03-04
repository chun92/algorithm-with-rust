use std::io;
use std::cmp::Ordering;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut iter = numbers.iter();
    let first = iter.next().unwrap();
    let second = iter.next().unwrap();

    match first.cmp(&second) {
        Ordering::Less => println!("<"),
        Ordering::Greater => println!(">"),
        Ordering::Equal => println!("==")
    }
}
