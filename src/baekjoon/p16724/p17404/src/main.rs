fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_numbers()[0];

    let mut dp = vec![vec![vec![usize::MAX; n]; 3]; 3];

    let (r, g, b) = {
        let v = read_line_as_numbers();
        (v[0], v[1], v[2])
    };
    dp[0][0][0] = r;
    dp[1][1][0] = g;
    dp[2][2][0] = b;

    for i in 1..n - 1 {
        let (r, g, b) = {
            let v = read_line_as_numbers();
            (v[0], v[1], v[2])
        };

        let min = std::cmp::min(dp[0][1][i - 1], dp[0][2][i - 1]);
        dp[0][0][i] = if min == usize::MAX { usize::MAX } else { min + r };
        let min = std::cmp::min(dp[1][1][i - 1], dp[1][2][i - 1]);
        dp[1][0][i] = if min == usize::MAX { usize::MAX } else { min + r };
        let min = std::cmp::min(dp[2][1][i - 1], dp[2][2][i - 1]);
        dp[2][0][i] = if min == usize::MAX { usize::MAX } else { min + r };

        let min = std::cmp::min(dp[0][0][i - 1], dp[0][2][i - 1]);
        dp[0][1][i] = if min == usize::MAX { usize::MAX } else { min + g };
        let min = std::cmp::min(dp[1][0][i - 1], dp[1][2][i - 1]);
        dp[1][1][i] = if min == usize::MAX { usize::MAX } else { min + g };
        let min = std::cmp::min(dp[2][0][i - 1], dp[2][2][i - 1]);
        dp[2][1][i] = if min == usize::MAX { usize::MAX } else { min + g };

        let min = std::cmp::min(dp[0][0][i - 1], dp[0][1][i - 1]);
        dp[0][2][i] = if min == usize::MAX { usize::MAX } else { min + b };
        let min = std::cmp::min(dp[1][0][i - 1], dp[1][1][i - 1]);
        dp[1][2][i] = if min == usize::MAX { usize::MAX } else { min + b };
        let min = std::cmp::min(dp[2][0][i - 1], dp[2][1][i - 1]);
        dp[2][2][i] = if min == usize::MAX { usize::MAX } else { min + b };
    }
    
    let (r, g, b) = {
        let v = read_line_as_numbers();
        (v[0], v[1], v[2])
    };

    let last_r = vec![dp[1][1][n - 2], dp[1][2][n - 2], dp[2][1][n - 2], dp[2][2][n - 2]].iter().min().unwrap() + r;
    let last_g = vec![dp[0][0][n - 2], dp[0][2][n - 2], dp[2][0][n - 2], dp[2][2][n - 2]].iter().min().unwrap() + g;
    let last_b = vec![dp[0][0][n - 2], dp[0][1][n - 2], dp[1][0][n - 2], dp[1][1][n - 2]].iter().min().unwrap() + b;

    let result_vec = vec![last_r, last_g, last_b];
    let result = result_vec.iter().min().unwrap();
    println!("{}", result);
}
