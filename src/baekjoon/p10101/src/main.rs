fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let (a, b, c) = (read_line_as_number(), read_line_as_number(), read_line_as_number());

    match (a, b, c) {
        (a, b, c) if a == 60 && b == 60 && c == 60 => println!("Equilateral"),
        (a, b, c) if a + b + c == 180 && (a == b) || (b == c) || (c == a) => println!("Isosceles"),
        (a, b, c) if a + b + c == 180 => println!("Scalene"),
        _ => println!("Error"),
    }
}
