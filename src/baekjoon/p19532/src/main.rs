fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let args = read_line_as_numbers();
    let (a, b, c, d, e, f) = (args[0], args[1], args[2], args[3], args[4], args[5]);
    let x = (c * e - b * f) / (a * e - b * d);
    let y = (c * d - a * f) / (b * d - a * e);
    println!("{} {}", x, y);
}

