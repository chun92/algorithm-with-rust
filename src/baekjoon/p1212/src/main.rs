use std::io::{stdin, stdout, BufWriter, Write};

fn read_line_as_string() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let s = read_line_as_string();
    let mut output = BufWriter::new(stdout());
    s.chars()
        .enumerate()
        .for_each(|(i, c)| {
            let c = c.to_digit(8).unwrap();
            if i == 0 {
                write!(output, "{c:b}").unwrap();
            } else {
                write!(output, "{c:03b}").unwrap();
            }
        });
}
