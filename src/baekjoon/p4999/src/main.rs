fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn main() {
    let first = read_line_as_string();
    let second = read_line_as_string();
    if first.len() >= second.len() {
        println!("go");
    } else {
        println!("no");
    }
}
