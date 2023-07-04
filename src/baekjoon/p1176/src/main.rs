fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (n , k) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut vec = Vec::new();
    for _ in 0..n {
        vec.push(read_line_as_numbers()[0]);
    }

    let mut dp = vec![vec![0; n]; 1 << n];

    for i in 0..n {
        dp[1 << i][i] = 1;
    }

    for mask in 1..1 << n {
        for i in 0..n {
            if mask & 1 << i == 0 {
                continue;
            }
            for j in 0..n {
                if mask & 1 << j != 0 {
                    continue;
                }
                let i_height = vec[i];
                let j_height = vec[j];
                if i_height.abs_diff(j_height) > k {
                    dp[mask | 1 << j][j] += dp[mask][i];
                }
            }
        }
    }

    let mut ans: u64 = 0;
    for i in 0..n {
        ans += dp[(1 << n) - 1][i];
    }

    println!("{}", ans);
}
