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
    let mut n = read_line_as_number();
    if n % 10 != 0 {
        println!("-1");
        return;
    }

    let m = n / 300;
    n -= m * 300;
    let s = n / 60;
    n -= s * 60;
    let b = n / 10;
    println!("{} {} {}", m, s, b);
}
