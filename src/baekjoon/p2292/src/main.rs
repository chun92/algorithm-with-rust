use std::io;

fn read_line_as_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse::<i32>().expect("Failed to parse input")
}

fn get_num(cur: i32, iter: i32) -> i32 {
    if cur - iter * 6 > 0 {
        get_num(cur - iter * 6, iter + 1)
    } else {
        iter
    }
}

fn main() {
    let n = read_line_as_number();

    let counter = {
        if n == 1 {
            1
        } else {
            get_num(n - 1, 1) + 1
        }
    };

    println!("{}", counter);
}
