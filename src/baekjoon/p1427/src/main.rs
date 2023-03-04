fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn main() {
    let input = read_line_as_string();
    let mut result: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    result.sort();
    result
        .iter()
        .rev()
        .for_each(|i| print!("{}", i));
    println!();
}
