use std::io::{stdin, stdout, Write, BufWriter};

fn read_line_as_number() -> usize {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let n = read_line_as_number();
    let mut output = BufWriter::new(stdout());

    for i in 0..n * 5 {
        for j in 0..n * 5 {
            match (j / n) % 5 {
                0 => write!(output, "@").unwrap(),
                1..=3 => match (i / n) % 5 {
                    i if i % 2 == 0 => write!(output, " ").unwrap(),
                    _ => write!(output, "@").unwrap(),
                },
                4 => write!(output, "@").unwrap(),
                _ => unreachable!(),
            }
        }
        writeln!(output).unwrap();
    }
}
