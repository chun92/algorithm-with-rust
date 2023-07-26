fn read_line_as_number() -> u32 {
    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text).unwrap();
    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => return i,
        Err(..) => return 0,
    };
}

fn main() {
    let mut sum = read_line_as_number();
    for _ in 0..9 {
        sum -= read_line_as_number();
    }
    println!("{}", sum);
}
