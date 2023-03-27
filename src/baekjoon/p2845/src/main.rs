fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let args = read_line_as_numbers();
    let (l, p) = (args[0], args[1]);
    let vec = read_line_as_numbers();
    vec
        .iter()
        .map(|&n| n - l * p)
        .for_each(|n| print!("{} ", n));
}
