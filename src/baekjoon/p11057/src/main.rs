fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let n = read_line_as_number();
    let mut memo = vec![vec![0; 10]; n];
    for i in 0..10 {
        memo[0][i] = 1;
    }

    for i in 0..n - 1 {
        for j in 0..10 {
            for k in j..10 {
                memo[i + 1][k] += memo[i][j];
            }
            memo[i + 1][j] %= 10007;
        }
    }

    let mut ans = 0;
    for i in 0..10 {
        ans += memo[n - 1][i];
    }
    println!("{}", ans % 10007);
}
