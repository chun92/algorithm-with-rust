fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (e, s, m) = {
        let v = read_line_as_numbers();
        (v[0] % 15, v[1], v[2] % 19)
    };

    let mut n = s;
    loop {
        if n % 15 == e && n % 19 == m {
            println!("{}", n);
            break;
        }
        n += 28;
    }
}
