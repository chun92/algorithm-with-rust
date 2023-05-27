fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let mut numbers = read_line_as_numbers();
    numbers.sort_unstable();
    println!("{}", numbers[1] + numbers[2]);
}
