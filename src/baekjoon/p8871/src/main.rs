fn read_line_as_number() -> i32 {
    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text).unwrap();
    let trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => return i,
        Err(..) => return 0,
    };
}

fn main() {
    let n = read_line_as_number();
    println!("{} {}", (n + 1) * 2, (n + 1) * 3);
}
