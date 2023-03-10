fn read_line_as<T>() -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn get_pow_with_mod(r: u64, i: u64, m: u64) -> u64 {
    let mut result = 1;
    let mut r = r;
    let mut i = i;

    while i > 0 {
        if i % 2 == 1 {
            result = (result * r) % m;
        }
        r = (r * r) % m;
        i /= 2;
    }

    result
}

fn main() {
    let _n: usize = read_line_as();
    let str = read_line_as::<String>();

    let r: u64 = 31;
    let m: u64 = 1234567891;

    let result = str
        .chars()
        .enumerate()
        .fold(0, |acc, (i, c)| {
            let n = c as u64 - 96;
            (get_pow_with_mod(r, i as u64, m) * n + acc) % m
        });

    println!("{}", result);
}
