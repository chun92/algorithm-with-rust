fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let a = read_line_as_numbers();
    let b = read_line_as_numbers();
    let sum_a = a.iter().sum::<usize>();
    let sum_b = b.iter().sum::<usize>();
    println!("{}", if sum_a >= sum_b { sum_a } else { sum_b });
}
