fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn dp(n: usize, memo: &mut Vec<usize>) -> usize {
    if memo[n] != 0 {
        return memo[n];
    }

    if n == 2 {
        memo[2] = 3;
        return 3;
    }


    let mut res = dp (n - 2, memo) * 3;
    for i in 2..n / 2 {
        res += dp(n - i * 2, memo) * 2
    }
    res += 2;
    memo[n] = res;
    res
}

fn main() {
    let n = read_line_as_number();

    if n % 2 == 1 {
        println!("0");
        return;
    }

    let mut memo = vec![0; n + 1];
    println!("{}", dp(n, &mut memo));
}
