fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let (a, b) = (read_line_as_number(), read_line_as_number());
    println!("{}", a * b);
}
