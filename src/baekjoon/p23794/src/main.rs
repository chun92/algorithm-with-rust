use std::io::{stdin, stdout, Write, BufWriter};

fn read_line_as_number() -> usize {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let n = read_line_as_number();
    let mut output = BufWriter::new(stdout());

    for i in 0..n + 2 {
        for j in 0..n + 2 {
            if i == 0 || i == n + 1 || j == 0 || j == n + 1 {
                write!(output, "@").unwrap();
            } else {
                write!(output, " ").unwrap();
            }
        }
        writeln!(output).unwrap();
    }
}
