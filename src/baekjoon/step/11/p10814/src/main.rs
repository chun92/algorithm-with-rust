use std::io;
use io::Write;

fn read_line_as_numbers() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Failed to parse number")
}

fn read_line_as_num_string_tuple() -> (i32, String) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let num = iter.next().unwrap().parse().expect("Failed to parse number");
    let string = iter.next().unwrap().to_string();
    (num, string)
}

fn print_result(vec: &Vec<(i32, String)>) {
    let mut out = io::BufWriter::new(io::stdout().lock());
    for (num, string) in vec {
        writeln!(out, "{} {}", num, string).unwrap();
    }
}

fn main() {
    let n = read_line_as_numbers();
    let mut vec = Vec::new();
    for _ in 0..n {
        vec.push(read_line_as_num_string_tuple());
    }
    vec.sort_by(|a, b| a.0.cmp(&b.0));

    print_result(&vec);
}
