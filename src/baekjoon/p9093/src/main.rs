use std::io::{stdin, stdout, BufWriter, Write};

fn read_line_as_number() -> usize {
    let mut input_text = String::new();
    stdin().read_line(&mut input_text).unwrap();
    let trimmed = input_text.trim();
    match trimmed.parse::<usize>() {
        Ok(i) => i,
        Err(..) => 0,
    }
}

fn read_line_as_strings() -> Vec<String> {
    let mut input_text = String::new();
    stdin().read_line(&mut input_text).unwrap();
    let trimmed = input_text.trim();
    trimmed.split_whitespace().map(|s| s.to_string()).collect()
}

fn main() {
    let mut output = BufWriter::new(stdout());
    let n = read_line_as_number();

    for _ in 0..n {
        let vec = read_line_as_strings();
        let result = vec
            .iter()
            .map(|s| s.chars().rev().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ");
        writeln!(output, "{}", result).unwrap();
    }
}
