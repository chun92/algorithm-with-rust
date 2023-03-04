fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        get_gcd(b, a % b)
    }
}

fn factorization(n: i32) -> Vec<i32> {
    let mut n = n;
    let mut factors = Vec::new();
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            factors.push(i);
            n /= i;
        } else {
            i += 1;
        }
    }
    if n != 1 {
        factors.push(n);
    }
    factors
}

fn main() {
    let _n = read_line_as_numbers()[0];
    let numbers = read_line_as_numbers();

    let mut num = 0;
    numbers
        .iter()
        .for_each(|x| {
            if num == 0 {
                num = *x;
            } else {
                let gcd = get_gcd(num, *x);
                num *= *x / gcd;
            }
        });
    
    let mut pow_num = 0;
    let mut is_pow = false;
    factorization(num)
        .iter()
        .for_each(|x| {
            if pow_num == 0 {
                pow_num = *x;
                is_pow = true;
            } else if pow_num != *x{
                is_pow = false;
            }
        });
    
    if is_pow {
        num *= pow_num;
    }

    println!("{}", num);
}
