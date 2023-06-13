fn read_line_as_number() -> u64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<u64>().unwrap()
}

fn get_digits_number(n: u64) -> usize {
    let mut i = 0;
    let mut n = n;
    while n >= 10 {
        n /= 10;
        i += 1;
    }
    i
}

type Nums = (u64, u64, u64, u64, u64, u64, u64, u64, u64);

fn f(n: u64, memo: &Vec<u64>) -> Nums {
    let mut result = (0, 0, 0, 0, 0, 0, 0, 0, 0);
    if n == 0 {
        return result;
    }
    
    let i = get_digits_number(n);

    let num = 10u64.pow(i as u32);
    let first = n / num;
    let rest = n % num;
    
    let first_num = memo[i] * first;

    result.0 += first_num + if first > 1 { 1 } else { 0 } * num + if first == 1 { rest + 1 } else { 0 };
    result.1 += first_num + if first > 2 { 1 } else { 0 } * num + if first == 2 { rest + 1 } else { 0 };
    result.2 += first_num + if first > 3 { 1 } else { 0 } * num + if first == 3 { rest + 1 } else { 0 };
    result.3 += first_num + if first > 4 { 1 } else { 0 } * num + if first == 4 { rest + 1 } else { 0 };
    result.4 += first_num + if first > 5 { 1 } else { 0 } * num + if first == 5 { rest + 1 } else { 0 };
    result.5 += first_num + if first > 6 { 1 } else { 0 } * num + if first == 6 { rest + 1 } else { 0 };
    result.6 += first_num + if first > 7 { 1 } else { 0 } * num + if first == 7 { rest + 1 } else { 0 };
    result.7 += first_num + if first > 8 { 1 } else { 0 } * num + if first == 8 { rest + 1 } else { 0 };
    result.8 += first_num + if first > 9 { 1 } else { 0 } * num + if first == 9 { rest + 1 } else { 0 };

    let next = f(rest, memo);

    result.0 += next.0;
    result.1 += next.1;
    result.2 += next.2;
    result.3 += next.3;
    result.4 += next.4;
    result.5 += next.5;
    result.6 += next.6;
    result.7 += next.7;
    result.8 += next.8;

    return result;
}

fn get_all_digits(n: u64, memo0: &Vec<u64>) -> u64 {
    let i = get_digits_number(n);
    let num = 10u64.pow(i as u32);
    (n - num + 1) * (i as u64 + 1) + memo0[i]
}

fn main() {
    let mut memo = vec![0; 12];
    memo[0] = 0; // 0
    for i in 1..11 {
        memo[i] = 10u64.pow(i as u32 - 1) + 10 * memo[i - 1];
    }

    let mut memo0 = vec![0; 12];
    memo0[0] = 0; // 0
    for i in 1..11 {
        memo0[i] = memo0[i - 1] + 10u64.pow(i as u32 - 1) * 9 * (i as u64);
    }

    let t = read_line_as_number();
    let result = f(t, &memo);
    let all_digits = get_all_digits(t, &memo0);
    let zeros = all_digits - result.0 - result.1 - result.2 - result.3 - result.4 - result.5 - result.6 - result.7 - result.8;
    println!("{} {} {} {} {} {} {} {} {} {}", zeros, result.0, result.1, result.2, result.3, result.4, result.5, result.6, result.7, result.8);
}
