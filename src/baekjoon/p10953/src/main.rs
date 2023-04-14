use std::io::{stdin, stdout, Write, BufWriter};

fn read_line_as_number() -> i32 {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn read_line_as_number_seperated_by_comma() -> Vec<i32> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_number();
    for _ in 0..n {
        let input = read_line_as_number_seperated_by_comma();
        let mut output = BufWriter::new(stdout());
        writeln!(output, "{}", input[0] + input[1]).unwrap();
    }
}
