use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut result = String::from(input.trim());
    result.push_str("??!");
    println!("{}", result);
}
