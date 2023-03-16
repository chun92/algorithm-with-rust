fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<usize>().unwrap()
}

fn dp_bottom_up(n: usize, vec: &Vec<usize>) -> usize {
    if n == 1 {
        return vec[0];
    }

    if n == 2 {
        return vec[0] + vec[1];
    }

    let mut dp = vec![(0, 0, 0, 0); n];
    dp[0] = (0, vec[0], 0, vec[0]);
    dp[1] = (vec[0], vec[1], vec[0] + vec[1], vec[0] + vec[1]);

    for i in 2..n {
        let zero = dp[i - 1].3;
        let first = dp[i - 1].0 + vec[i];
        let second = dp[i - 1].1 + vec[i];
        let third = std::cmp::max(zero, std::cmp::max(first, second));

        dp[i as usize] = (zero, first, second, third);
    }

    dp.iter().max_by_key(|x| x.3).unwrap().3
}

fn main() {
    let n = read_line_as_number();
    let mut vec = Vec::new();
    for _ in 0..n {
        vec.push(read_line_as_number());
    }

    println!("{}", dp_bottom_up(n, &vec));
}


