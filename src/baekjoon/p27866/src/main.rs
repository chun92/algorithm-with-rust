fn read_line_as<T>() -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn main() {
    let str = read_line_as::<String>();
    let n = read_line_as::<usize>();
    println!("{}", str.chars().nth(n - 1).unwrap());
}
