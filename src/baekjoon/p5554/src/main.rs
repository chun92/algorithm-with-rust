fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let (a, b, c, d) = (read_line_as_number(), read_line_as_number(), read_line_as_number(), read_line_as_number());
    let sum = a + b + c + d;
    println!("{}", sum / 60);
    println!("{}", sum % 60);
}
