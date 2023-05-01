fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn read_line_as_strings() -> Vec<String> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim()
        .chars()
        .map(|s| s.to_string())
        .collect()
}

fn main() {
    let (a, b, c) = {
        let mut v = read_line_as_numbers();
        v.sort();
        (v[0], v[1], v[2])
    };

    let v = read_line_as_strings();
    let (x, y, z) = {
        (v[0].as_str(), v[1].as_str(), v[2].as_str())
    };

    match (x, y, z) {
        ("A", "B", "C") => println!("{} {} {}", a, b, c),
        ("A", "C", "B") => println!("{} {} {}", a, c, b),
        ("B", "A", "C") => println!("{} {} {}", b, a, c),
        ("B", "C", "A") => println!("{} {} {}", b, c, a),
        ("C", "A", "B") => println!("{} {} {}", c, a, b),
        ("C", "B", "A") => println!("{} {} {}", c, b, a),
        _ => unreachable!(),
    }
}
