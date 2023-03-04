fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn print_goldbach(primes: &Vec<bool>, n: i32) {
    for i in n / 2 .. n {
        if primes[(n - i) as usize] && primes[i as usize] {
            println!("{} {}", n - i, i);
            break;
        }
    }
}

fn set_primes(primes: &mut Vec<bool>) {
    primes[0] = false;
    primes[1] = false;

    let limit = (primes.len() as f64).sqrt().ceil() as usize;
    for i in 2..=limit {
        if primes[i] {
            let mut j = i * 2;
            while j < primes.len() {
                primes[j] = false;
                j += i;
            }
        }
    }
}

fn main() {
    let n = read_line_as_number();

    let mut primes = vec![true; 10001];
    set_primes(&mut primes);

    for _ in 0..n {
        let target = read_line_as_number();
        print_goldbach(&primes, target);
    }
}
