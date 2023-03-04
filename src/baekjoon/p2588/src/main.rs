use std::io;

fn read_line_as_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn get_digit_number(number: i32, digit: i32) -> i32 {
    (number / 10_i32.pow(digit as u32)) % 10
}

fn main() {
    let first = read_line_as_number();
    let second = read_line_as_number();

    println!("{}", first * (get_digit_number(second, 0)));
    println!("{}", first * (get_digit_number(second, 1)));
    println!("{}", first * (get_digit_number(second, 2)));
    println!("{}", first * second);
}