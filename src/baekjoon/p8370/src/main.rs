fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (n1, k1, n2, k2) = {
        let line = read_line_as_numbers();
        (line[0], line[1], line[2], line[3])
    };

    println!("{}", n1 * k1 + n2 * k2);
}
