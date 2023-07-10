fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_numbers()[0];
    let mut memo = vec![0; n + 2];

    for i in 1..=n {
        let (t, p) = {
            let line = read_line_as_numbers();
            (line[0], line[1])
        };

        memo[i + 1] = std::cmp::max(memo[i + 1], memo[i]);
        if i + t <= n + 1 {
            memo[i + t] = std::cmp::max(memo[i + t], memo[i] + p);
        }
    }

    println!("{}", memo[n + 1]);
}
