fn read_line_as_number() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_number()[0];
    let mut memo: Vec<u64> = vec![0; n + 1];

    memo[0] = 0;
    memo[1] = 1;
    for i in 2..=n {
        memo[i] = memo[i - 1] + memo[i - 2];
    }
    println!("{}", memo[n]);
}
