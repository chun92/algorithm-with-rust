fn read_line_as_number() -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<u32>().unwrap()
}

fn main() {
    let n = read_line_as_number();
    let result = 2_u32.pow(n);
    println!("{}", result);
}
