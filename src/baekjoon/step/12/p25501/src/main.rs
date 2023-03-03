fn read_line_as<T>() -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn recursion(s: &String, l: usize, r: usize) -> (usize, usize) {
    if l >= r {
        (1, 1)
    } else if s.chars().nth(l).unwrap() != s.chars().nth(r).unwrap() {
        (0, 1)
    } else {
        let res = recursion(s, l + 1, r - 1);
        (res.0, res.1 + 1)
    }
}

fn is_palindrome(s: String) -> (usize, usize) {
    recursion(&s, 0, s.len() - 1)
}

fn main() {
    let n: usize = read_line_as();
    for _ in 0..n {
        let s: String = read_line_as();
        let res = is_palindrome(s);
        println!("{} {}", res.0, res.1);
    }
}
