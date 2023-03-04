fn read_line_as_number() -> u64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<u64>().unwrap()
}

fn main() {
    let n = read_line_as_number();
    println!("{}", n * n);
    println!("2");
}
