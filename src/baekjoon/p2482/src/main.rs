use std::io::*;

fn read() -> usize {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse().unwrap()
}

const MOD: i64 = 1_000_000_003;
const MAX: usize = 1_001;
fn main() {
    let n = read();
    let k = read();

    let mut dp = vec![vec![0; MAX]; MAX]; 
    for i in 1..=n {
        dp[i][1] = i as i64;
        dp[i][0] = 1;
    }

    for i in 2..=n {
        for j in 2..=k {
            dp[i][j] = (dp[i - 1][j] + dp[i - 2][j - 1]) % MOD;
        }
    }

    if k == 1 {
        println!("{}", n as i64 % MOD);
    } else {
        let result = (dp[n - 3][k - 1] + dp[n - 1][k]) % MOD;
        println!("{}", result);
    }
}
