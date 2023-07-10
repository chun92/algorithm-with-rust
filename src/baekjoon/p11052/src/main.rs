fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_numbers()[0];
    let vec = read_line_as_numbers();
    let mut memo = vec![0; n + 1];
    for i in 0..n {
        for (j, v) in vec.iter().enumerate() {
            let j = j + 1;
            if i + j <= n {
                memo[i + j] = memo[i + j].max(memo[i] + v);
            }
        }
    }

    println!("{}", memo[n]);
}
