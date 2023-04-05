use std::collections::HashSet;

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
    let n: usize = read_line_as();
    let mut set = HashSet::new();
    let mut sum = 0;

    for _ in 0..n {
        let s: String = read_line_as();
        match s {
            s if s == "ENTER" => {
                sum += set.len();
                set.clear();
            },
            s => {
                set.insert(s);
            }
        }
    }
    sum += set.len();

    println!("{}", sum);
}
