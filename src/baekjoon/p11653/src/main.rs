fn read_line_as_usize() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

fn get_prime_factors(num: usize) -> Vec<usize> {
    let mut factors = Vec::new();
    get_prime_factors_impl(num, &mut factors);
    return factors;
}

fn get_prime_factors_impl(num: usize, factors: &mut Vec<usize>) {
    match num {
        1 => (),
        n => {
            let mut i = 2;
            let limit = (n as f64).sqrt().ceil() as usize;
            while i <= limit {
                if n % i == 0 {
                    factors.push(i);
                    get_prime_factors_impl(num / i, factors);
                    return ();
                }
                i += 1;
            };
            factors.push(n);
            return ();
        }
    }
}

fn main() {
    let num = read_line_as_usize();
    let factors = get_prime_factors(num);
    factors
        .iter()
        .for_each(|factor| println!("{}", factor));
}
