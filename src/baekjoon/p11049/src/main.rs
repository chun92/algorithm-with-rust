fn read_line_as_numbers() -> Vec<u32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_numbers()[0] as usize;
    let mut dp = vec![vec![(0, 0, 0); n]; n];

    for i in 0..n {
        let numbers = read_line_as_numbers();
        dp[i][i] = (numbers[0], numbers[1], 0);
    }

    for diff in 1..n {
        for i in 0..n - diff {
            let mut vec = Vec::new();
            let j = i + diff;
            for k in i..j {
                let left = dp[i][k];
                let right = dp[k + 1][j];
                let calc = left.0 * left.1 * right.1 + left.2 + right.2;
                vec.push(calc);
            }
            let min = *vec.iter().min().unwrap();
            dp[i][j] = (dp[i][i].0, dp[j][j].1, min);
        }
    }

    println!("{}", dp[0][n - 1].2)
}
