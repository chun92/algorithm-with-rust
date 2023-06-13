fn read_line_as_numbers() -> Vec<u64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_n_is_power_of_two(n: u64) -> (bool, u64) {
    let mut i = 0;
    let mut n = n;
    let mut is_power_of_two = true;
    while n > 1 {
        if n % 2 == 1 {
            is_power_of_two = false;
        }
        n /= 2;
        i += 1;
    }
    (is_power_of_two, i)
}

fn f(n: u64, memo: &Vec<u64>) -> u64 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    } 
    
    let (is_power_of_two, i) = get_n_is_power_of_two(n);
    if is_power_of_two {
        return memo[i as usize];
    }

    let num = 2u64.pow(i as u32);
    memo[i as usize] + f(n - num, memo) + n - num
}

fn main() {
    let (a, b) = {
        let line = read_line_as_numbers();
        (line[0].max(line[1]), line[0].min(line[1]))
    };

    let mut memo = vec![0; 59];
    memo[0] = 1;
    for i in 1..59 {
        memo[i] = 2u64.pow(i as u32 - 1) + 2 * memo[i - 1] - 1;
    }

    println!("{}", f(a, &memo) - f(b - 1, &memo));
}
