use std::io::{self, BufRead, Write};
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let next = lines.next().unwrap();
    let mut nm = split::<usize>(&next);
    let n = nm.next().unwrap();
    let m = nm.next().unwrap();

    let mut arr = vec![vec![0; m + 1]; n + 1];
    let mut dp = vec![vec![0; m + 1]; n + 1];

    for i in 1..=n {
        let next = lines.next().unwrap();
        let mut row = split::<i32>(&next);
        for j in 1..=m {
            arr[i][j] = row.next().unwrap();
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1] - dp[i - 1][j - 1] + arr[i][j];
        }
    }

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let k = lines.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..k {
        let next = lines.next().unwrap();
        let mut ijxy = split::<usize>(&next);
        let i = ijxy.next().unwrap();
        let j = ijxy.next().unwrap();
        let x = ijxy.next().unwrap();
        let y = ijxy.next().unwrap();
        writeln!(out, "{}", dp[x][y] - dp[i - 1][y] - dp[x][j - 1] + dp[i - 1][j - 1]).unwrap();
    }
}

fn split<T: FromStr>(s: &String) -> impl Iterator<Item = T> + '_ {
    s.split_whitespace().map(|item| item.parse().ok().unwrap())
}
