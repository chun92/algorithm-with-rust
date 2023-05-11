use std::io;

fn main() {
    let mut s1 = String::new();
    io::stdin().read_line(&mut s1).unwrap();
    let mut s2 = String::new();
    io::stdin().read_line(&mut s2).unwrap();

    let s1 = s1.trim();
    let s2 = s2.trim();

    let l1 = s1.len();
    let l2 = s2.len();
    let mut dp = vec![vec![0; l2 + 1]; l1 + 1];

    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();

    for i in 0..l1 {
        for j in 0..l2 {
            if chars1[i] == chars2[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = std::cmp::max(dp[i + 1][j], dp[i][j + 1]);
            }
        }
    }

    println!("{}", dp[l1][l2]);

    let mut ans = String::new();
    let (mut i, mut j) = (l1, l2);
    while i > 0 && j > 0 {
        if dp[i][j] == dp[i - 1][j] {
            i -= 1;
        } else if dp[i][j] == dp[i][j - 1] {
            j -= 1;
        } else {
            ans.push(chars1[i - 1]);
            i -= 1;
            j -= 1;
        }
    }

    println!("{}", ans.chars().rev().collect::<String>());
}
