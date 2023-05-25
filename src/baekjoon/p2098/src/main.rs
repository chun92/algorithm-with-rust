use std::io;
use std::cmp;
use std::io::prelude::*;

const INF: i32 = 1_000_000_000;
const MAX: usize = 16;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut w: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let costs: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        w.push(costs);
    }
    
    let mut dp: Vec<Vec<i32>> = vec![vec![INF; MAX]; 1<<MAX];
    dp[(1<<0)][0] = 0;
    for mask in 1..(1<<n) {
        for i in 0..n {
            if (mask & (1<<i)) != 0 {
                for j in 0..n {
                    if (mask & (1<<j)) == 0 && w[i][j] != 0 {
                        dp[mask|(1<<j)][j] = cmp::min(dp[mask|(1<<j)][j], dp[mask][i] + w[i][j]);
                    }
                }
            }
        }
    }
    
    let mut res = INF;
    for i in 0..n {
        if w[i][0] != 0 {
            res = cmp::min(res, dp[(1<<n)-1][i] + w[i][0]);
        }
    }
    
    println!("{}", res);
}
