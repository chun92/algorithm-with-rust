fn read_line_as_numbers() -> Vec<u128> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn combination(n: u128, m: u128) -> u128 {
    let mut ans = 1;
    for i in 0..m {
        ans *= n - i;
        ans /= i + 1;
    }
    ans
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    println!("{}", combination(n, m));
}
