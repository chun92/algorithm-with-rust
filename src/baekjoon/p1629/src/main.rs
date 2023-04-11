use std::io;

fn read_line_as_numbers() -> Vec<u64> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn power_modulo(mut base: u64, mut exponent: u64, modulo: u64) -> u64 {
    let mut result = 1;

    base %= modulo;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulo;
        }
        exponent >>= 1;
        base = (base * base) % modulo;
    }

    result
}

fn main() {
    let (a, b, c) = {
        let args = read_line_as_numbers();
        (args[0], args[1], args[2])
    };

    let result = power_modulo(a, b, c);
    println!("{}", result);
}
