fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn solve() -> usize {
    let n = read_line_as_numbers()[0];
    let mut vec = Vec::new();
    for _ in 0..n {
        let v = read_line_as_numbers();
        vec.push((v[0], v[1]));
    }

    vec.sort_by(|a, b| a.0.cmp(&b.0));
    
    let mut ans = 0;
    let mut index = usize::MAX;
    for i in 0..n {
        if vec[i].1 < index {
            index = vec[i].1;
            ans += 1;
        }
    }
    ans
}

fn main() {
    let test_cases = read_line_as_numbers()[0];
    for _ in 0..test_cases {
        println!("{}", solve());
    }
}
