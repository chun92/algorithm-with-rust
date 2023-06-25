fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let sum = read_line_as_number();
    let diff = read_line_as_number();

    let a = (sum + diff) / 2;
    let b = sum - a;

    println!("{}", a);
    println!("{}", b);
}
