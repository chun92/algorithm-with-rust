fn read_line_as_numbers() -> Vec<i64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_numbers()[0] as usize;
    let nums = read_line_as_numbers();
    let (b, c) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut ans = 0;
    for i in 0..n {
        let mut num = nums[i];
        num -= b;
        ans += 1;
        if num > 0 {
            ans += (num as f64 / c as f64).ceil() as i64;
        }
    }
    println!("{}", ans);
}
