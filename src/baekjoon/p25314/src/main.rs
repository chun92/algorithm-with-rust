use std::io;

fn read_line_as_num() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let iter = read_line_as_num() / 4;

    for _ in 0..iter {
        print!("long ");
    }
    println!("int");
}
