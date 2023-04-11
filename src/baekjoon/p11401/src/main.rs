const MOD: u64 = 1_000_000_007;
fn read_line_as_numbers() -> Vec<u64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn mod_pow(mut base: u64, mut exp: u64, modulo: u64) -> u64 {
    let mut result = 1;
    base %= modulo;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulo;
        }
        exp >>= 1;
        base = (base * base) % modulo;
    }
    result
}

fn factorial_mod(n: u64, modulo: u64) -> u64 {
    let mut result = 1;
    for i in 1..=n {
        result = (result * i) % modulo;
    }
    result
}

fn combination_mod(n: u64, k: u64, modulo: u64) -> u64 {
    let num = factorial_mod(n, modulo);
    let den1 = mod_pow(factorial_mod(k, modulo), modulo - 2, modulo);
    let den2 = mod_pow(factorial_mod(n - k, modulo), modulo - 2, modulo);

    num * den1 % modulo * den2 % modulo
}

fn main() {
    let (n, k) = {
        let numbers = read_line_as_numbers();
        (numbers[0], numbers[1])
    };
    let result = combination_mod(n, k, MOD);
    println!("{}", result);
}
