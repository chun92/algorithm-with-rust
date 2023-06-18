fn read_line_as_number() -> usize {
    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text).unwrap();
    let trimmed = input_text.trim();
    match trimmed.parse::<usize>() {
        Ok(i) => return i,
        Err(..) => return 0,
    };
}

fn main() {
    let n = read_line_as_number();

    let mut memo = vec![vec![vec![0; 1 << 10]; 10]; n + 1];
    
    for i in 1..10 {
        memo[1][i][1 << i] = 1;
    }

    for i in 2..n + 1 {
        for j in 0..10 {
            for k in 0..1 << 10 {
                if j == 0 {
                    memo[i][j][k | (1 << j)] = (memo[i][j][k | (1 << j)] + memo[i - 1][j + 1][k]) % 1000000000;
                } else if j == 9 {
                    memo[i][j][k | (1 << j)] = (memo[i][j][k | (1 << j)] + memo[i - 1][j - 1][k]) % 1000000000;
                } else {
                    memo[i][j][k | (1 << j)] = (memo[i][j][k | (1 << j)] + memo[i - 1][j - 1][k] + memo[i - 1][j + 1][k]) % 1000000000;
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..10 {
        ans = (ans + memo[n][i][(1 << 10) - 1]) % 1000000000;
    }
    println!("{}", ans);
}
