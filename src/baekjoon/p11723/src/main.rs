use std::io::{Write, BufWriter, stdout, stdin};

fn read_line_as_strings() -> Vec<String> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace().map(|s| s.to_string()).collect()
}

fn read_line_as_number() -> i32 {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

fn parse_command_and_apply(command: &Vec<String>, vec: &mut Vec<i32>) -> Option<i32> {
    let command_op = command[0].as_str();
    match command_op {
        "add" => {
            let index: usize = command[1].parse().unwrap();
            vec[index] = 1;
            None
        },
        "remove" => {
            let index: usize = command[1].parse().unwrap();
            vec[index] = 0;
            None
        },
        "check" => {
            let index: usize = command[1].parse().unwrap();
            Some(vec[index])
        },
        "toggle" => {
            let index: usize = command[1].parse().unwrap();
            if vec[index] == 0 {
                vec[index] = 1;
            } else {
                vec[index] = 0;
            }
            None
        },
        "all" => {
            for i in 0..vec.len() {
                vec[i] = 1;
            }
            None
        },
        "empty" => {
            for i in 0..vec.len() {
                vec[i] = 0;
            }
            None
        },
        _ => {
            panic!("Invalid command");
        }
    }
}

fn main() {
    let n = read_line_as_number();

    let mut vec = vec![0; 21];
    let mut out = BufWriter::new(stdout());

    for _ in 0..n {
        let command = read_line_as_strings();
        let result = parse_command_and_apply(&command, &mut vec);
        match result {
            Some(n) => {
                writeln!(out, "{}", n).unwrap();
            },
            None => {}
        }
    }
}