fn read_line_as_numbers() -> Vec<i64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let numbers = read_line_as_numbers();
    let (n, m) = (numbers[0], numbers[1]);
    let result = (n - m).abs();
    println!("{}", result);
}
