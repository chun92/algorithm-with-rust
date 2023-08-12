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

    let mut y = 0;
    let mut m = 0;

    for i in 0..n {
        let v = vec[i];
        y += v / 30 * 10 + 10;
        m += v / 60 * 15 + 15;
    }

    if m < y {
        println!("M {}", m);
    } else if m > y {
        println!("Y {}", y);
    } else {
        println!("Y M {}", m);
    }
}
