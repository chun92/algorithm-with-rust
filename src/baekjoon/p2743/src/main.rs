use std::io;

fn read_line_len() -> i32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let value = s.trim().len();
    value.try_into().unwrap()
}

fn main() {
    let num = read_line_len();
    println!("{}", num);
}
