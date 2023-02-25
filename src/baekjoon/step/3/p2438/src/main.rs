use std::io;
fn read_line_as_number() -> i32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let value = s.trim().parse::<i32>().unwrap();
    return value;
}

fn main() {
    let count = read_line_as_number();
    for i in 1..=count {
        for _ in 0..i {
            print!("*");
        }
        print!("\n");
    }
}
