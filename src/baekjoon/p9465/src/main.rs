fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let test_cases = read_line_as_numbers()[0];
    for _ in 0..test_cases {
        let n = read_line_as_numbers()[0];
        let up = read_line_as_numbers();
        let down = read_line_as_numbers();

        let mut memo = vec![vec![0; 3]; n];

        for i in 0..n {
            if i == 0 {
                memo[i][0] = 0;
                memo[i][1] = up[i];
                memo[i][2] = down[i];
            } else {
                memo[i][0] = std::cmp::max(memo[i - 1][0], std::cmp::max(memo[i - 1][1], memo[i - 1][2]));
                memo[i][1] = std::cmp::max(memo[i - 1][0], memo[i - 1][2]) + up[i];
                memo[i][2] = std::cmp::max(memo[i - 1][0], memo[i - 1][1]) + down[i];
            }
        }

        println!("{}", std::cmp::max(memo[n - 1][0], std::cmp::max(memo[n - 1][1], memo[n - 1][2])));
    }
}
