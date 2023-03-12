fn read_line_as_number() -> u64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<u64>().unwrap()
}

fn is_prime(n: u64) -> bool {
    let sqrt = (n as f64).sqrt().ceil() as u64;
    for i in 2..=sqrt {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn get_next_prime(n: u64) -> u64 {
    if n == 0 || n == 1 || n == 2 {
        return 2;
    }

    if is_prime(n) {
        return n;
    } else if n % 2 == 0 {
        return get_next_prime(n + 1);
    } else {
        return get_next_prime(n + 2);
    }
}

fn main() {
    let n = read_line_as_number();
    for _ in 0..n {
        let m = read_line_as_number();
        println!("{}", get_next_prime(m));
    }
}
