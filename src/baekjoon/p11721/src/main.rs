fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    read_line_as_string()
        .chars()
        .collect::<Vec<char>>()
        .chunks(10)
        .for_each(|c| println!("{}", c.iter().collect::<String>()));
}
