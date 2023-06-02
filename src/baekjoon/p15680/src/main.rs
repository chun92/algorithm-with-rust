fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    match read_line_as_number() {
        0 => println!("YONSEI"),
        1 => println!("Leading the Way to the Future"),
        _ => unreachable!(),
    }
}
