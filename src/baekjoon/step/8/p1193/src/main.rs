use std::io;

fn read_line_as_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse::<i32>().expect("Failed to parse number")
}

fn calc_position_impl(cur: i32, iter: i32) -> (i32, i32) {
    if cur <= iter {
        (iter, cur)
    } else {
        calc_position_impl(cur - iter, iter + 1)
    }
}

fn calc_position(n: i32) -> (i32, i32) {
    let (iter, index) = calc_position_impl(n, 0);
    if iter % 2 == 0 {
        (index, iter - index + 1)
    } else {
        (iter - index + 1, index)
    }
}

fn main() {
    let n = read_line_as_number();
    let (a, b) = calc_position(n);
    println!("{}/{}", a, b);
}
