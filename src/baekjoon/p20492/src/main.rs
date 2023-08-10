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

    let num1 = n / 100 * 78;
    let num2 = n / 100 * 80 + n / 100 * 20 * 78 / 100;
    println!("{} {}", num1, num2);
}
