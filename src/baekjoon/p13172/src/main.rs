fn read_line_as_numbers() -> Vec<u64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

const MOD: u64 = 1_000_000_007;

fn mod_pow(x: u64, n: u64) -> u64 {
    if n == 0 {
        1
    } else if n % 2 == 0 {
        mod_pow(x * x % MOD, n / 2)
    } else {
        x * mod_pow(x, n - 1) % MOD
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn irreducible_fraction(a: u64, b: u64) -> (u64, u64) {
    let g = gcd(a, b);
    (a / g, b / g)
}

fn mod_inverse(a: u64) -> u64 {
    mod_pow(a, MOD - 2)
}

fn mod_div(a: u64, b: u64) -> u64 {
    a * mod_inverse(b) % MOD
}

fn get_div(a: u64, b: u64) -> u64 {
    let (a, b) = irreducible_fraction(a, b);
    mod_div(a, b)
}

fn main() {
    let m = read_line_as_numbers()[0];

    let mut sum = 0;

    for _ in 0..m {
        let (a, b) = {
            let v = read_line_as_numbers();
            (v[0], v[1])
        };
        sum += get_div(b, a);
        sum %= MOD;
    }
    println!("{}", sum);
}
