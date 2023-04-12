fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let n = read_line_as_number();

    for i in (-n + 1)..(n) {
        let abs_i = n - i.abs();
        for _ in 0..abs_i {
            print!("*");
        }
        for _ in 0..(2 * (n - abs_i)) {
            print!(" ");
        }
        for _ in 0..abs_i {
            print!("*");
        }
        println!();
    }
}
