use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    
    let mut iter = numbers.iter();
    let first = iter.next().unwrap();
    let second = iter.next().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let time = input.trim().parse::<i32>().unwrap();

    let minute = *second + time;
    let add_hour = minute / 60;
    let minute = minute % 60;
    let hour = (*first + add_hour) % 24;
    println!("{} {}", hour, minute);
}
