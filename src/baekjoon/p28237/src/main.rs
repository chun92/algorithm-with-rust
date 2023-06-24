fn read_line_as_number() -> i64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn get_divisors(n: i64) -> Vec<i64> {
    let mut divisors = Vec::new();
    let idx = (n as f64).sqrt() as i64;
    for i in 1..=idx {
        if n % i == 0 {
            divisors.push(i);
        }
    }
    
    let mut idx = divisors.len() as i64 - 1;
    if divisors[idx as usize] * divisors[idx as usize] == n {
        idx -= 1;
    }
    while idx >= 0 {
        divisors.push(n / divisors[idx as usize]);
        idx -= 1;
    }
    divisors
}

fn solve(n: i64) -> bool {
    let divisors1 = get_divisors(n);
    let divisors2 = get_divisors(n + 2);

    for d1 in &divisors1 {
        for d2 in &divisors2 {
            let a = *d1;
            let c = n / a;
            let b = *d2;
            let d = -(n + 2) / b;

            if a * d + b * c == n + 1 {
                println!("{} {} {} {}", a, b, c, d);
                return true;
            }

            let a = -*d1;
            let c = n / a;
            let b = *d2;
            let d = -(n + 2) / b;

            if a * d + b * c == n + 1 {
                println!("{} {} {} {}", a, b, c, d);
                return true;
            }

            let a = *d1;
            let c = n / a;
            let b = -*d2;
            let d = -(n + 2) / b;

            if a * d + b * c == n + 1 {
                println!("{} {} {} {}", a, b, c, d);
                return true;
            }

            let a = -*d1;
            let c = n / a;
            let b = -*d2;
            let d = -(n + 2) / b;

            if a * d + b * c == n + 1 {
                println!("{} {} {} {}", a, b, c, d);
                return true;
            }
        }
    }
    false
}

fn main() {
    let n = read_line_as_number();
    let result = solve(n);
    if !result {
        println!("-1");
    }
}
