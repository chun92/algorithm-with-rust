fn read_line_as_numbers() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let (n, _m) = {
        let i = read_line_as_numbers();
        (i[0], i[1])
    };

    for _ in 0..n {
        let str = read_line_as_string();
        let result = str.chars().rev().collect::<String>();
        println!("{}", result);
    }
}