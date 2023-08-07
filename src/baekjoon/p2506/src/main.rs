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
    let mut sum = 0;
    let mut cur = 1;
    for i in 0..n {
        let x = vec[i];
        if x == 1 {
            sum += cur;
            cur += 1;
        } else {
            cur = 1;
        }
    }
    println!("{}", sum);
}
