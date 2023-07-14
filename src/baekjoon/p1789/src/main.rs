fn read_line_as_number() -> u64 {
    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
    let trimmed = input_text.trim();
    match trimmed.parse::<u64>() {
        Ok(i) => i,
        Err(..) => 0,
    }
}

fn main() {
    let s = read_line_as_number();
    for i in 1.. {
        let n = i * i + i;
        if n > 2 * s {
            println!("{}", i - 1);
            break;
        }
    }
}
