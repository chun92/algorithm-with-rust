fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (mut n, mut m, mut k) = {
        let v = read_line_as_numbers();
        (v[0], v[1], v[2])
    };

    while k > 0 {
        if n > m * 2 {
            n -= 1;
        } else {
            m -= 1;
        }
        k -= 1;
    }

    println!("{}", std::cmp::min(n / 2, m));
}
