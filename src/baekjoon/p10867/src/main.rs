fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
fn main() {
    let _ = read_line_as_numbers();
    let mut numbers = read_line_as_numbers();
    numbers.sort_unstable();
    numbers.dedup();
    numbers.iter().for_each(|n| print!("{} ", n));
}