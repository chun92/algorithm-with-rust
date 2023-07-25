fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let str = read_line_as_string();
    println!("{}", str.chars().rev().collect::<String>());
}
