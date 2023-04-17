fn read_line_as_number() -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<u32>().unwrap()
}

fn main() {
    let mut sum = 0;
    for _ in 0..5 {
        sum += read_line_as_number();
    }
    println!("{}", sum);
}
