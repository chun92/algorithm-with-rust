fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let first = read_line_as_number();
    let second = read_line_as_number();
    println!("{}", 2 * second - first);
}
