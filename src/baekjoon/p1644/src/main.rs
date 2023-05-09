fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn eratostenes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut primes = vec![];
    for i in 2..=n {
        if is_prime[i] {
            primes.push(i);
            let mut j = 2 * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    primes
}

fn main() {
    let n = read_line_as_number();
    let primes = eratostenes(n);

    if primes.len() == 0 {
        println!("0");
        return;
    }

    let mut ans = 0;
    let mut iter1 = primes.iter();
    let mut iter2 = primes.iter();

    let mut p1 = *iter1.next().unwrap();
    let mut p2 = *iter2.next().unwrap();
    let mut sum = p1;

    loop {
        if p1 > p2 {
            break;
        }
        if sum == n {
            ans += 1;
        }
        
        if sum < n {
            match iter2.next() {
                Some(p) => {
                    p2 = *p;
                    sum += p2;
                }
                None => {
                    break;
                }
            }
        } else {
            sum -= p1;
            match iter1.next() {
                Some(p) => {
                    p1 = *p;
                }
                None => {
                    break;
                }
            }
        }
    }

    println!("{}", ans);
}
