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

fn print_result(vec: &Vec<String>) {
    for s in vec {
        println!("{}", s);
    }
}

fn main() {
    let n: usize = read_line_as();
    let mut set: HashSet<String> = HashSet::new();
    for _ in 0..n {
        let s: String = read_line_as();
        set.insert(s);
    }
    let mut strings:Vec<String> = set
        .into_iter()
        .collect();
    strings.sort_unstable_by(|a, b| {
        match a.len().cmp(&b.len()) {
            std::cmp::Ordering::Equal => a.cmp(b),
            ord => ord,
        }
    });

    print_result(&strings);
}