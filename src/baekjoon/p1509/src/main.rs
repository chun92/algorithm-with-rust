fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}


fn fill_memo(memo: &mut Vec<Vec<bool>>, chars: &Vec<char>, n: usize) {
    for i in 0..n {
        memo[i][i] = true;
    }
    
    for i in 0..n - 1 {
        if chars[i] == chars[i + 1] {
            memo[i][i + 1] = true;
        }
    }

    for i in 2..n {
        for j in 0..n - i {
            if chars[j] == chars[j + i] && memo[j + 1][j + i - 1] {
                memo[j][j + i] = true;
            }
        }
    }
}

fn main() {
    let str = read_line_as_string();
    let chars: Vec<char> = str.chars().collect();
    let n = str.len();
    let mut memo = vec![vec![false; n]; n];
    fill_memo(&mut memo, &chars, n);

    let mut dp = vec![0; n + 1];
    dp[0] = 0;
    for i in 1..=n {
        dp[i] = dp[i - 1] + 1;
        for j in 0..i {
            if memo[j][i - 1] {
                dp[i] = std::cmp::min(dp[i], dp[j] + 1);
            }
        }
    }
    println!("{}", dp[n]);
}
