fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (r1, s) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };
    println!("{}", 2 * s - r1);
}
