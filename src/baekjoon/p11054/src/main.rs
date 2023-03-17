fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn solve(n: i32, vec: &Vec<i32>) -> i32 {
    if n == 1 {
        return 1;
    }
    let mut dp = Vec::new();
    dp.push((1, 1));

    for i in 1..n as usize {
        let mut up_max = 0;
        for j in 0..i {
            if vec[j] < vec[i] {
                up_max = std::cmp::max(up_max, dp[j].0);
            }
        }

        let mut down_max = 0;
        for j in 0..i {
            if vec[j] > vec[i]  {
                let n = std::cmp::max(dp[j].0, dp[j].1);
                down_max = std::cmp::max(down_max, n);
            }
        }
        dp.push((up_max + 1, down_max + 1));
    }

    let up_max = dp.iter().max_by_key(|x| x.0).unwrap().0;
    let down_max = dp.iter().max_by_key(|x| x.1).unwrap().1;
    std::cmp::max(up_max, down_max)
}

fn main() {
    let n = read_line_as_numbers()[0];
    let vec = read_line_as_numbers();
    let result = solve(n, &vec);
    println!("{}", result);
}
