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

    for i in 0..5 * n {
        for j in 0..5 * n {
            let i = i / n % 5;
            let j = j / n % 5;

            if j == 0 {
                write!(writer, "@").unwrap();
            } else if i == 0 || i == 4 {
                if j == 4 {
                    write!(writer, "@").unwrap();
                } else {
                    write!(writer, " ").unwrap();
                }
            } else if i == 1 || i == 3 {
                if j == 3 {
                    write!(writer, "@").unwrap();
                } else if j == 1 || j == 2 {
                    write!(writer, " ").unwrap();
                }
            } else if i == 2 {
                if j == 1 || j == 2 {
                    write!(writer, "@").unwrap();
                }
            }
        }
        writeln!(writer).unwrap();
    }
}
