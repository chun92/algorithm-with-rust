fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn print_result(a: usize, b: usize) {
    let a_result = match a {
        1 => 5000000,
        2..=3 => 3000000,
        4..=6 => 2000000,
        7..=10 => 500000,
        11..=15 => 300000,
        16..=21 => 100000,
        _ => 0,
    };

    let b_result = match b {
        1 => 5120000,
        2..=3 => 2560000,
        4..=7 => 1280000,
        8..=15 => 640000,
        16..=31 => 320000,
        _ => 0,
    };

    println!("{}", a_result + b_result);
}

fn main() {
    let t = read_line_as_numbers()[0];
    for _ in 0..t {
        let (a, b) = {
            let v = read_line_as_numbers();
            (v[0], v[1])
        };
        print_result(a, b);
    }
}
