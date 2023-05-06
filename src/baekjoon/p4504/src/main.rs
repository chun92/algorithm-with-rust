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
    let n = read_line_as_number();
    loop {
        let num = read_line_as_number();
        if num == 0 {
            break;
        }
        if num % n == 0 {
            println!("{} is a multiple of {}.", num, n);
        } else {
            println!("{} is NOT a multiple of {}.", num, n);
        }
    }
}
