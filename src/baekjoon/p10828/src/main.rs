use std::io::{stdout, BufWriter, Write};

enum Command {
    PUSH(i32),
    POP,
    SIZE,
    EMPTY,
    TOP,
}

fn read_line_as_strings() -> Vec<String> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace().map(|s| s.to_string()).collect()
}

fn get_command(vec: Vec<String>) -> Command {
    match vec[0].as_str() {
        "push" => Command::PUSH(vec[1].parse::<i32>().unwrap()),
        "pop" => Command::POP,
        "size" => Command::SIZE,
        "empty" => Command::EMPTY,
        "top" => Command::TOP,
        _ => panic!("Invalid command"),
    }
}

fn main() {
    let n = read_line_as_strings()[0].parse::<i32>().unwrap();

    let mut vec = Vec::new();

    let mut out = BufWriter::new(stdout());

    for _ in 0..n {
        let line = read_line_as_strings();
        let command = get_command(line);

        match command {
            Command::PUSH(x) => vec.push(x),
            Command::POP => {
                if vec.is_empty() {
                    writeln!(out, "-1").unwrap();
                } else {
                    writeln!(out, "{}", vec.pop().unwrap()).unwrap();
                }
            }
            Command::SIZE => writeln!(out, "{}", vec.len()).unwrap(),
            Command::EMPTY => {
                if vec.is_empty() {
                    writeln!(out, "1").unwrap();
                } else {
                    writeln!(out, "0").unwrap();
                }
            }
            Command::TOP => {
                if vec.is_empty() {
                    writeln!(out, "-1").unwrap();
                } else {
                    writeln!(out, "{}", vec.last().unwrap()).unwrap();
                }
            }
        }
    }
}
