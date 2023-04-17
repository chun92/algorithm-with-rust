fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let n = read_line_as_string().parse::<usize>().unwrap();

    let mut result = read_line_as_string();
    for _ in 1..n {
        result = result
            .chars()
            .zip(read_line_as_string().chars())
            .map(|(a, b)| if a == b { a } else { '?' })
            .collect::<String>()
    }

    println!("{}", result);
}
