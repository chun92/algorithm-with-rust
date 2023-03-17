fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn lcs(s1: String, s2: String) -> i32 {
    let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];

    for i in 1..=s1.len() {
        for j in 1..=s2.len() {
            if s1[i - 1..i] == s2[j - 1..j] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }

    dp[s1.len()][s2.len()]
}
fn main() {
    let s1 = read_line_as_string();
    let s2 = read_line_as_string();
    println!("{}", lcs(s1, s2));
}
