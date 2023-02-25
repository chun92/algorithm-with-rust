use std::io;

fn read_line_as_number() -> i32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let value = s.trim().parse::<i32>().unwrap();
    value
}

fn read_line_first_last() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let first = s.trim().chars().nth(0).unwrap();
    let last = s.trim().chars().nth(s.trim().len() - 1).unwrap();
    format!("{}{}", first, last)
}

fn main() {
    let num = read_line_as_number();
    for _ in 0..num {
        let result = read_line_first_last();
        println!("{}", result);
    }
}
