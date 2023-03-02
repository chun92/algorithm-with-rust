use std::io;
use io::Write;

fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<usize>().unwrap()
}

fn main() {
    let n = read_line_as_number();
    let mut vec = vec![0; 10001];

    for _ in 0..n {
        let num = read_line_as_number();
        vec[num] += 1;
    }

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    vec
        .iter()
        .enumerate()
        .for_each(|(i, &v)| {
            for _ in 0..v {
                writeln!(out, "{}", i).unwrap();
            }
        });
}
