fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let _n = read_line_as_numbers()[0];
    let mut vec1 = read_line_as_numbers();
    let mut vec2 = read_line_as_numbers();
    vec1.sort_unstable();
    vec2.sort_unstable_by(|a, b| b.cmp(a));

    let answer = vec1
        .iter()
        .zip(vec2.iter())
        .map(|(a, b)| a * b)
        .sum::<usize>();

    println!("{}", answer);
}