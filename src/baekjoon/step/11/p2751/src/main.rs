use std::io;
use io::Write;

fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let n = read_line_as_number();
    let mut vec = Vec::new();
    for _ in 0..n {
        vec.push(read_line_as_number());
    }

    vec.sort_unstable();
    
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    vec
        .iter()
        .for_each(|x| writeln!(out, "{}", x).unwrap());
}
