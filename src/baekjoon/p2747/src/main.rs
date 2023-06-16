fn read_line_as_number() -> u32 {
    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text).unwrap();
    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => return i,
        Err(..) => return 0,
    };
}

fn fibonacci(n: u32, memo: &mut Vec<u32>) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    if memo[n as usize] != 0 {
        return memo[n as usize];
    }

    memo[n as usize] = fibonacci(n - 1, memo) + fibonacci(n - 2, memo);
    return memo[n as usize];
}

fn main() {
    let n = read_line_as_number();
    let mut memo = vec![0; n as usize + 1];
    println!("{}", fibonacci(n, &mut memo));
}
