use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let result = input.trim().parse::<i32>().unwrap() - 543;
    println!("{}", result);
}
