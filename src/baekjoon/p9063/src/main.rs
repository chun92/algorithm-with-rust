use std::i32::{MAX, MIN};

fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_numbers()[0];

    let mut x_max = MIN;
    let mut x_min = MAX;
    let mut y_max = MIN;
    let mut y_min = MAX;

    for _ in 0..n {
        let (x, y) = {
            let v = read_line_as_numbers();
            (v[0], v[1])
        };

        x_max = std::cmp::max(x_max, x);
        x_min = std::cmp::min(x_min, x);
        y_max = std::cmp::max(y_max, y);
        y_min = std::cmp::min(y_min, y);
    }

    let ans = (x_max - x_min) * (y_max - y_min);
    println!("{}", ans);    
}
