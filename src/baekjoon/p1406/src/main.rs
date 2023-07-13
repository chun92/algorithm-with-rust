use std::io::{stdin, stdout, Write, BufWriter};

fn read_line_as_string() -> Vec<char> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().chars().collect()
}

fn read_line_as_strings() -> Vec<String> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace().map(|s| s.to_string()).collect()
}

fn main() {
    let mut original = read_line_as_string();
    let mut stack = Vec::new();

    let n = read_line_as_strings()[0].parse::<usize>().unwrap();
    for _ in 0..n {
        let commands = read_line_as_strings();
        match commands[0].as_str() {
            "L" => {
                if original.len() > 0 {
                    stack.push(original.pop().unwrap());
                }
            },
            "D" => {
                if stack.len() > 0 {
                    original.push(stack.pop().unwrap());
                }
            },
            "B" => {
                if original.len() > 0 {
                    original.pop();
                }
            },
            "P" => {
                original.push(commands[1].chars().nth(0).unwrap());
            },
            _ => panic!("Invalid command")
        }
    }

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    for c in original {
        write!(writer, "{}", c).unwrap();
    }
    while let Some(c) = stack.pop() {
        write!(writer, "{}", c).unwrap();
    }
    write!(writer, "\n").unwrap();
}
