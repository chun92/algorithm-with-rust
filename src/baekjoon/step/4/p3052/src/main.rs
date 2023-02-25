use std::io;
use std::collections::HashSet;

fn read_line_as_number() -> i32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let value = s.trim().parse::<i32>().unwrap();
    value
}

fn main() {
    let mut set = HashSet::new();
    for _ in 0..10 {
        let num = read_line_as_number();
        set.insert(num % 42);
    }

    println!("{}", set.len());
}
