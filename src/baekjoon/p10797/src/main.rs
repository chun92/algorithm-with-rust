fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_numbers()[0];
    let target = read_line_as_numbers();
    let result = target
        .iter()
        .fold(0, |acc, x| acc + if n == *x { 1 } else { 0 });
    println!("{}", result);
}
