use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let number = input.trim().parse::<i32>().unwrap();

    let result = match number {
        x if x % 4 == 0 && x % 100 != 0 => 1,
        x if x % 400 == 0 => 1,
        _ => 0
    };

    println!("{}", result);
}
