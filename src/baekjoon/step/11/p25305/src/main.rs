fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let arguments = read_line_as_numbers();
    let (_n, k) = (arguments[0], arguments[1]);
    let mut numbers = read_line_as_numbers();
    numbers.sort();
    let results = numbers
        .into_iter()
        .rev()
        .collect::<Vec<i32>>();

    println!("{}", results[k as usize - 1]);
}
