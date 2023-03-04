use std::io;

fn read_line_as_ascii() -> u8 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let value = s.trim().chars().nth(0).unwrap() as u8;
    value
}

fn main() {
    let num = read_line_as_ascii();
    println!("{}", num);
}
