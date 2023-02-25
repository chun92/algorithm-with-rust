use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let first = input.trim().parse::<i32>().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let second = input.trim().parse::<i32>().unwrap();

    let result = match (first, second) {
        (f, s) if f > 0 && s > 0 => 1,
        (f, s) if f < 0 && s > 0 => 2,
        (f, s) if f < 0 && s < 0 => 3,
        _ => 4
    };

    println!("{}", result);
}
