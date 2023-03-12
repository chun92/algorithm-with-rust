use std::collections::HashSet;

fn read_line_as_number() -> u64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn eratos_chest(n: u64) -> Vec<u64> {
    let mut primes = vec![true; n as usize];
    primes[0] = false;
    primes[1] = false;
    for i in 2..n {
        if primes[i as usize] {
            let mut j = i * i;
            while j < n {
                primes[j as usize] = false;
                j += i;
            }
        }
    }
    primes
        .iter()
        .enumerate()
        .filter(|(_, &is_prime)| is_prime)
        .map(|(i, _)| i as u64)
        .collect()
}

fn main() {
    let n = read_line_as_number();
    let primes = eratos_chest(1_000_000);
    let primes_hash = primes.iter().map(|&x| x as u64).collect::<HashSet<u64>>();

    for _ in 0..n {
        let m = read_line_as_number();
        let mut i = 0;
        let mut result = HashSet::new();
        while primes[i] < m {
            let other = m - primes[i];
            if primes_hash.contains(&other) {
                result.insert((std::cmp::min(primes[i], other), std::cmp::max(primes[i], other)));
            }
            i += 1;
        }
        println!("{}", result.len());
    }
}