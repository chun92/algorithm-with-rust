fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let a = read_line_as_number();
    let b = read_line_as_number();
    let c = read_line_as_number();
    let mid = a + b + c - std::cmp::max(a, std::cmp::max(b, c)) - std::cmp::min(a, std::cmp::min(b, c));
    println!("{}", mid);
}
