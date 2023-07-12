fn read_line_as_string() -> Option<String> {
    let mut input = String::new();
    let n = std::io::stdin().read_line(&mut input).unwrap();
    if n == 0 {
        None
    } else {
        Some(input.trim_end_matches('\n').to_string())
    }
}

fn main() {
    while let Some(s) = read_line_as_string() {
        println!("{}", s);
    }
}
