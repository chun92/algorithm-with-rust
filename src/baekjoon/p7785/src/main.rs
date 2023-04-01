use std::io::{stdin, stdout, BufWriter, Write};
use std::collections::HashSet;

fn read_line_as<T>() -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().parse::<T>().unwrap()
}

fn read_line_as_strings() -> Vec<String> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace().map(|s| s.to_string()).collect()
}

fn main() {
    let n: usize = read_line_as();
    let mut set = HashSet::new();

    for _ in 0..n {
        let s = read_line_as_strings();
        let name = s.get(0).unwrap();
        match s.get(1) {
            Some(s) if s == "enter" => set.insert(name.clone()),
            Some(s) if s == "leave" => set.remove(name),
            _ => panic!(),
        };
    }

    let mut out = BufWriter::new(stdout());
    let mut v: Vec<String> = set.into_iter().collect();
    v.sort_by(|a, b| b.cmp(a));
    v.iter().for_each(|s| {
        writeln!(out, "{}", s).unwrap(); 
    });
}
