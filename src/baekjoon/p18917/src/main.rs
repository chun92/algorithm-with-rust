use std::io::{stdin, stdout, BufWriter, Write};

fn read_line_as_numbers() -> Vec<u64> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
fn main() {
    let mut sum = 0;
    let mut xor_sum = 0;
    let mut output = BufWriter::new(stdout());
    let lines = read_line_as_numbers()[0];
    for _ in 0..lines {
        let numbers = read_line_as_numbers();
        let command = numbers[0];
        match command {
            1 => {
                let x = numbers[1];
                sum += x;
                xor_sum ^= x;
            },
            2 => {
                let x = numbers[1];
                sum -= x;
                xor_sum ^= x;
            },
            3 => {
                writeln!(output, "{}", sum).unwrap();
            },
            4 => {
                writeln!(output, "{}", xor_sum).unwrap();
            },
            _ => {
                panic!("Unknown command: {}", command);
            }
        }
    }
}
