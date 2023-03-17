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
    dp.push(1);

    for i in 1..n as usize {
        let mut max = 0;
        for j in 0..i {
            if vec[j] < vec[i] {
                max = std::cmp::max(max, dp[j]);
            }
        }
        dp.push(max + 1);
    }

    *dp.iter().max().unwrap()
}

fn main() {
    let n = read_line_as_numbers()[0];
    let vec = read_line_as_numbers();
    let result = solve(n, &vec);
    println!("{}", result);
}
