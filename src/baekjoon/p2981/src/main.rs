use std::io::{BufWriter, Write, stdout};

fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn get_gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        get_gcd(b, a % b)
    }
}

fn get_divisors(n: i32) -> Vec<i32> {
    let mut divisors = Vec::new();
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            divisors.push(i);
            if i != n / i {
                divisors.push(n / i);
            }
        }
        i += 1;
    }
    divisors.push(n);
    divisors.sort();
    divisors
}

fn main() {
    let n = read_line_as_number()  as usize;
    let mut vec = Vec::new();
    for _ in 0..n {
        let num = read_line_as_number();
        vec.push(num);
    }
    vec.sort_unstable_by(|a, b| b.cmp(a));

    let mut sub_vec = Vec::new();
    for i in 0..n - 1 {
        sub_vec.push(vec[i] - vec[i + 1]);
    }

    let gcd = sub_vec.iter().fold(sub_vec[0], |acc, x| get_gcd(acc, *x));
    let divisors = get_divisors(gcd);

    let mut out = BufWriter::new(stdout());

    divisors
        .iter()
        .enumerate()
        .for_each(|(i, x)| {
            if i == 0 {
                write!(out, "{}", x).unwrap();
            } else {
                write!(out, " {}", x).unwrap();
            }
        });
    writeln!(out).unwrap();
}
