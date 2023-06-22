fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (_n, m, k) = {
        let v = read_line_as_numbers();
        (v[0], v[1], v[2])
    };

    let mut min = std::usize::MAX;
    let mut result = 0;
    for i in 0..k {
        let (f, d) = {
            let v = read_line_as_numbers();
            (v[0], v[1])
        };
        let dist = f + m + 1 - d;
        if dist < min {
            min = dist;
            result = i + 1;
        }
    }
    println!("{}", result);
}
