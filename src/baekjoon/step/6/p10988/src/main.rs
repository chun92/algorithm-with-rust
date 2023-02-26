use std::io;

fn get_line() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input = input.trim().try_into().unwrap();
    let result = input
        .chars()
        .zip(input.chars().rev())
        .fold(true, |res, (x, y)| {
            res && x == y
        });
    
    match result {
        true => println!("1"),
        false => println!("0")
    }
}

fn main() {
    get_line();
}
