fn read_line_as_numbers() -> Vec<u32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let tests = read_line_as_numbers()[0];
    for _ in 0..tests {
        let n = read_line_as_numbers()[0] as usize;
        let numbers = read_line_as_numbers();

        let mut dp = vec![vec![0; n]; n];
        let mut sum = vec![vec![0; n]; n];

        for i in 0..n {
            dp[i][i] = numbers[i];
        }

        for diff in 1..n {
            for i in 0..n - diff {
                let mut vec = Vec::new();
                let j = i + diff;
                for k in i..j {
                    let sum = sum[i][k] + sum[k + 1][j];
                    let dp = dp[i][k] + dp[k + 1][j];
                    vec.push((sum + dp, dp));
                }
                let min = *vec.iter().min_by_key(|x| x.0).unwrap();
                dp[i][j] = min.1;
                sum[i][j] = min.0;
            }
        }
        println!("{}", sum[0][n - 1]);
    }
}
