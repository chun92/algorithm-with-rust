fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let nums = read_line_as_numbers();
    let mut result = nums
        .iter()
        .fold(0, |acc, x| acc + x.pow(2));
    result %= 10;
    println!("{}", result);
}
