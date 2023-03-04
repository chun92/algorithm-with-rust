use std::io;

fn read_line_as_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse::<i32>().expect("Failed to parse number")
}

fn get_res(n: i32) -> (i32, i32) {
    let x_5 = n / 5;

    for i in (0..=x_5).rev() {
        let remainder = n - i * 5;
        if remainder % 3 == 0 {
            return (i, remainder / 3)
        }
    }

    return (-1, -1)
}

fn main() {
    let n = read_line_as_number();
    match get_res(n) {
        (-1, -1) => println!("-1"),
        (x, y) => println!("{}", x + y)
    }
}
