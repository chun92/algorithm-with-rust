fn read_line_as_numbers() -> Vec<u64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (n, m) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let vec = read_line_as_numbers();
    let mut sums = vec![0; n as usize + 1];
    for i in 0..n {
        sums[i as usize + 1] = sums[i as usize] + vec[i as usize];
    }

    let mut count = 0;
    for i in 0..n {
        for j in i + 1..n + 1 {
            if sums[j as usize] - sums[i as usize] == m {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
