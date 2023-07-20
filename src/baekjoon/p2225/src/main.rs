fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn dp(memo: &mut Vec<Vec<usize>>, n: usize, k: usize) -> usize {
    if memo[n][k] > 0 {
        return memo[n][k];
    }

    if n == 0 || k == 1 {
        return 1;
    }

    let mut sum = 0;
    for i in 0..n + 1 {
        sum += dp(memo, n - i, k - 1);
    }
    sum %= 1_000_000_000;
    memo[n][k] = sum;
    sum
}

fn main() {
    let (n, k) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut memo = vec![vec![0; k + 1]; n + 1];
    println!("{}", dp(&mut memo, n, k));
}
