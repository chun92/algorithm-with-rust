fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let n = read_line_as_number();
    match n % 2 {
        0 => println!("SK"),
        1 => println!("CY"),
        _ => panic!("Invalid number"),
    }
}
