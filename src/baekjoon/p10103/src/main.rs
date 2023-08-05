fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_numbers()[0];
    let mut a = 100;
    let mut b = 100;
    for _ in 0..n {
        let (ax, bx) = {
            let v = read_line_as_numbers();
            (v[0], v[1])
        };

        if ax > bx {
            b -= ax;
        } else if ax < bx {
            a -= bx;
        }
    }

    println!("{}", a);
    println!("{}", b);
}
