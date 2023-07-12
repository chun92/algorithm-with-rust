fn read_line_as_number() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_number()[0];

    let mut memo = vec![vec![0_u64; n + 1]; 2];
    memo[0][1] = 1;
    memo[1][1] = 1;

    for i in 2..=n {
        memo[0][i] = memo[0][i - 1] + memo[1][i - 1];
        memo[1][i] = memo[0][i - 1];
    }

    println!("{}", memo[1][n]);
}
