fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let arguments = read_line_as_numbers();
    let (x, y, w, h) = (arguments[0], arguments[1], arguments[2], arguments[3]);
    let result = vec![x, y, w - x, h - y].into_iter().min().unwrap();
    println!("{}", result);
}
