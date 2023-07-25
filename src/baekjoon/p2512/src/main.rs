fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_numbers()[0];
    let mut vec = read_line_as_numbers();
    vec.sort();
    let mut sum = read_line_as_numbers()[0];

    for i in 0..n {
        let num = vec[i] * (n - i);
        if num > sum {
            println!("{}", (sum as f64 / (n - i) as f64).floor() as usize);
            return;
        }
        if i == n - 1 {
            println!("{}", vec[i]);
            return;
        }
        sum -= vec[i];
    }
}
