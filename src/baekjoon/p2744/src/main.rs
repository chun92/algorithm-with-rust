fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn main() {
    let input = read_line_as_string();
    let result = input
        .chars()
        .into_iter()
        .map(|x| {
            if x.is_ascii_uppercase() {
                x.to_ascii_lowercase()
            } else {
                x.to_ascii_uppercase()
            }
        })
        .collect::<String>();
    println!("{}", result);
}
