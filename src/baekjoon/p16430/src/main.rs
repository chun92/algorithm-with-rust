fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (a, b) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    println!("{} {}", b - a, b);
}
