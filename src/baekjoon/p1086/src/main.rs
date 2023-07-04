fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    n * factorial(n - 1)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn main() {
    let n = read_line_as_string().parse::<usize>().unwrap();
    let mut vec = Vec::new();

    for _ in 0..n {
        vec.push(read_line_as_string());
    }

    let k = read_line_as_string().parse::<u32>().unwrap();

    let vec = vec
        .iter()
        .map(|str| {
            let mut cur = 0;
            let mut digit = 1;
            for c in str.chars() {
                cur = (cur * 10 + c.to_digit(10).unwrap()) % k;
                digit = (digit * 10) % k;
            }
            (cur, digit)
        }).collect::<Vec<(u32, u32)>>();

    let mut dp = vec![vec![0_u64; k as usize]; 1 << n];

    for i in 0..n {
        let modular = vec[i].0 as usize;
        dp[1 << i][modular] = 1;
    }

    for mask in 0..(1 << n) {
        for i in 0..n {
            if mask & (1 << i) != 0 {
                continue;
            }

            for j in 0..k {
                let count = dp[mask][j as usize];
                if count == 0 {
                    continue;
                }
                let modular = (((j * vec[i].1) + vec[i].0) % k) as usize;
                dp[mask | (1 << i)][modular] += count;
            }
        }
    }

    let numerator = dp[(1 << n) - 1][0];
    let denominator = factorial(n as u64);
    if numerator == 0 {
        println!("0/1");
        return;
    }

    let gcd = gcd(numerator, denominator);
    println!("{}/{}", numerator / gcd, denominator / gcd);
}
