use std::io::{stdin, stdout, Write, BufWriter};
use std::collections::{HashSet, BTreeSet};

fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn read_lien_as_string() -> String {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let arguments = read_line_as_numbers();
    let (n, m) = (arguments[0], arguments[1]);
    
    let mut n_set = HashSet::new();
    let mut m_set = HashSet::new();

    for _ in 0..n {
        n_set.insert(read_lien_as_string());
    }

    for _ in 0..m {
        m_set.insert(read_lien_as_string());
    }

    let mut out = BufWriter::new(stdout());
    let result_set: BTreeSet<&String> = n_set
        .intersection(&m_set)
        .collect();

    writeln!(out, "{}", result_set.len()).unwrap();
    result_set
        .iter()
        .for_each(|s| writeln!(out, "{}", s).unwrap());
}
