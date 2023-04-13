use std::io::{stdin, stdout, Write, BufWriter};

fn read_line_as_number() -> usize {
    let mut input = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let n = read_line_as_number();
    let mut writer = BufWriter::new(stdout());

    for i in 0..n {
        for j in 0..(2 * n) {
            if (i + j) % 2 == 0 {
                write!(writer, "*").unwrap();
            } else {
                write!(writer, " ").unwrap();
            }
        }
        writeln!(writer).unwrap();
    }
}
