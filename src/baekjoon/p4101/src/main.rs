fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    loop {
        let numbers = read_line_as_numbers();
        let (a, b) = (numbers[0], numbers[1]);
        match (a, b) {
            (0, 0) => break,
            (a, b) if a > b => println!("Yes"),
            _ => println!("No"),
        }
    }
}
