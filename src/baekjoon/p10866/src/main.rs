use std::{io::{stdout, BufWriter, Write}, collections::VecDeque};

enum Command {
    PUSH_FRONT(i32),
    PUSH_BACK(i32),
    POP_FRONT,
    POP_BACK,
    SIZE,
    EMPTY,
    FRONT,
    BACK,
}

fn read_line_as_strings() -> Vec<String> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace().map(|s| s.to_string()).collect()
}

fn get_command(vec: Vec<String>) -> Command {
    match vec[0].as_str() {
        "push_front" => Command::PUSH_FRONT(vec[1].parse::<i32>().unwrap()),
        "push_back" => Command::PUSH_BACK(vec[1].parse::<i32>().unwrap()),
        "pop_front" => Command::POP_FRONT,
        "pop_back" => Command::POP_BACK,
        "size" => Command::SIZE,
        "empty" => Command::EMPTY,
        "front" => Command::FRONT,
        "back" => Command::BACK,
        _ => panic!("Invalid command"),
    }
}

fn main() {
    let n = read_line_as_strings()[0].parse::<i32>().unwrap();

    let mut vec_deque = VecDeque::new();

    let mut out = BufWriter::new(stdout());

    for _ in 0..n {
        let line = read_line_as_strings();
        let command = get_command(line);

        match command {
            Command::PUSH_FRONT(x) => vec_deque.push_front(x),
            Command::PUSH_BACK(x) => vec_deque.push_back(x),
            Command::POP_FRONT => {
                if vec_deque.is_empty() {
                    writeln!(out, "-1").unwrap();
                } else {
                    writeln!(out, "{}", vec_deque.pop_front().unwrap()).unwrap();
                }
            },
            Command::POP_BACK => {
                if vec_deque.is_empty() {
                    writeln!(out, "-1").unwrap();
                } else {
                    writeln!(out, "{}", vec_deque.pop_back().unwrap()).unwrap();
                }
            },
            Command::SIZE => writeln!(out, "{}", vec_deque.len()).unwrap(),
            Command::EMPTY => {
                if vec_deque.is_empty() {
                    writeln!(out, "1").unwrap();
                } else {
                    writeln!(out, "0").unwrap();
                }
            }
            Command::FRONT => {
                if vec_deque.is_empty() {
                    writeln!(out, "-1").unwrap();
                } else {
                    writeln!(out, "{}", vec_deque.front().unwrap()).unwrap();
                }
            }
            Command::BACK => {
                if vec_deque.is_empty() {
                    writeln!(out, "-1").unwrap();
                } else {
                    writeln!(out, "{}", vec_deque.back().unwrap()).unwrap();
                }
            }
        }
    }
}
