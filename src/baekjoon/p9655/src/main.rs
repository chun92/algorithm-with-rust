fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let n = read_line_as_number();
    match n % 4 {
        0 | 2 => println!("CY"),
        1 | 3 => println!("SK"),
        _ => panic!("Invalid number"),
    }
}
