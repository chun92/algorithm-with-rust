fn read_line_as_number() -> u64 {
    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text).unwrap();
    let trimmed = input_text.trim();
    match trimmed.parse::<u64>() {
        Ok(i) => i,
        Err(..) => 0,
    }
}

fn main() {
    let n = read_line_as_number();
    println!("{}", n * n * n);
}
