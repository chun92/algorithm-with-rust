fn read_line_as_number() -> usize {
    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text).unwrap();
    let trimmed = input_text.trim();
    match trimmed.parse::<usize>() {
        Ok(i) => return i,
        Err(..) => return 0,
    };
}

fn dp(n: usize, memo: &mut Vec<usize>) -> usize {
    if memo[n] != 0 {
        return memo[n];
    }

    let sqrt_n = (n as f64).sqrt() as usize;
    let mut min = usize::MAX;
    for i in (1..=sqrt_n).rev() {
        let remain = n - i * i;
        if remain == 0 {
            memo[n] = 1;
            return 1;
        } else {
            let tmp = dp(remain, memo);
            if tmp < min {
                min = tmp;
            }
        }
    }
    memo[n] = min + 1;
    return min + 1;
}

fn main() {
    let n = read_line_as_number();
    let mut memo = vec![0; n + 1];
    println!("{}", dp(n, &mut memo));
}
