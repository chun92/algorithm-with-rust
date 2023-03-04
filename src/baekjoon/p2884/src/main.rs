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

    let minute = *second - 45;
    if minute < 0 {
        let minute = minute + 60;
        let hour = *first -1;
        if hour < 0 {
            let hour = hour + 24;
            println!("{} {}", hour, minute);
        } else {
            println!("{} {}", hour, minute);
        }
    } else {
        println!("{} {}", *first, minute);
    }
}
