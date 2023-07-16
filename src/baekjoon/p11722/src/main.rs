fn read_line_as_numbers() -> Vec<u16> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let _ = read_line_as_numbers();
    let numbers = read_line_as_numbers();
    let n = numbers.len();
    let mut dp = vec![0; n];
    for i in 0..n {
        dp[i] = 1;
        for j in 0..i {
            if numbers[j] > numbers[i] {
                dp[i] = std::cmp::max(dp[i], dp[j] + 1);
            }
        }
    }
    println!("{}", dp.iter().max().unwrap());
}
