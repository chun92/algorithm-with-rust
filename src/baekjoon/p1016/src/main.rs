fn read_line_as_numbers() -> Vec<u64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn eratosphenes_sieve(max: u64) -> Vec<u64> {
    let mut sieve = vec![true; max as usize + 1];
    sieve[0] = false;
    sieve[1] = false;
    let limit = (max as f64).sqrt() as u64;
    for i in 2..limit+1 {
        if sieve[i as usize] {
            let mut j = i * i;
            while j <= max {
                sieve[j as usize] = false;
                j += i;
            }
        }
    }
    sieve
        .iter()
        .enumerate()
        .filter(|(_, &is_prime)| is_prime)
        .map(|(i, _)| (i * i) as u64)
        .collect()
}

fn another_sives(min: u64, max: u64, primes_sqaures: &Vec<u64>) -> Vec<u64> {
    let mut sieve = vec![true; (max - min + 1) as usize];
    for &prime_square in primes_sqaures {
        if prime_square > max {
            break;
        }
        let mut j = (min / prime_square) * prime_square;
        if j < min {
            j += prime_square;
        }
        while j <= max {
            sieve[(j - min) as usize] = false;
            j += prime_square;
        }
    }
    sieve
        .iter()
        .enumerate()
        .filter(|(_, &is_prime)| is_prime)
        .map(|(i, _)| i as u64 + min)
        .collect()
}

fn main() {
    let (min, max) = {
        let numbers = read_line_as_numbers();
        (numbers[0], numbers[1])
    };

    let primes_sqaures = eratosphenes_sieve(1000000);
    let res = another_sives(min, max, &primes_sqaures);
    println!("{}", res.len());
}
