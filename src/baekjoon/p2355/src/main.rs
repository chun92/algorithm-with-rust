fn read_line_as_numbers() -> Vec<i64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        if v[0] < v[1] {
            (v[0], v[1])
        } else {
            (v[1], v[0])
        }
    };

    let result = (m + n) * (m - n + 1) / 2;
    println!("{}", result);
}
