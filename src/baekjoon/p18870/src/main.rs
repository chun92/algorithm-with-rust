use std::io;
use io::Write;
use std::collections::HashMap;

fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let _n = read_line_as_numbers()[0];
    let vec = read_line_as_numbers();

    let mut map = HashMap::new();
    let mut sorted_vec = vec.clone();
    sorted_vec.sort();
    let mut count = 0;
    sorted_vec
        .iter()
        .for_each(|x| {
            if map.get(x).is_none() {
                map.insert(x, count);
                count += 1;
            }
        });
    
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    vec
        .iter()
        .enumerate()
        .for_each(|(i, x)| {
            if i != vec.len() - 1 {
                write!(out, "{} ", map.get(x).unwrap()).unwrap();
            } else {
                writeln!(out, "{}", map.get(x).unwrap()).unwrap();
            }
        });
}
