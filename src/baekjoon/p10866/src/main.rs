use std::{io::{stdout, BufWriter, Write}, collections::VecDeque};

enum Command {
    PushFront(i32),
    PushBack(i32),
    PopFront,
    PopBack,
    Size,
    Empty,
    Front,
    Back,
}

fn read_line_as_strings() -> Vec<String> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace().map(|s| s.to_string()).collect()
}

fn get_command(vec: Vec<String>) -> Command {
    match vec[0].as_str() {
        "push_front" => Command::PushFront(vec[1].parse::<i32>().unwrap()),
        "push_back" => Command::PushBack(vec[1].parse::<i32>().unwrap()),
        "pop_front" => Command::PopFront,
        "pop_back" => Command::PopBack,
        "size" => Command::Size,
        "empty" => Command::Empty,
        "front" => Command::Front,
        "back" => Command::Back,
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
            Command::PushFront(x) => vec_deque.push_front(x),
            Command::PushBack(x) => vec_deque.push_back(x),
            Command::PopFront => {
                if vec_deque.is_empty() {
                    writeln!(out, "-1").unwrap();
                } else {
                    writeln!(out, "{}", vec_deque.pop_front().unwrap()).unwrap();
                }
            },
            Command::PopBack => {
                if vec_deque.is_empty() {
                    writeln!(out, "-1").unwrap();
                } else {
                    writeln!(out, "{}", vec_deque.pop_back().unwrap()).unwrap();
                }
            },
            Command::Size => writeln!(out, "{}", vec_deque.len()).unwrap(),
            Command::Empty => {
                if vec_deque.is_empty() {
                    writeln!(out, "1").unwrap();
                } else {
                    writeln!(out, "0").unwrap();
                }
            }
            Command::Front => {
                if vec_deque.is_empty() {
                    writeln!(out, "-1").unwrap();
                } else {
                    writeln!(out, "{}", vec_deque.front().unwrap()).unwrap();
                }
            }
            Command::Back => {
                if vec_deque.is_empty() {
                    writeln!(out, "-1").unwrap();
                } else {
                    writeln!(out, "{}", vec_deque.back().unwrap()).unwrap();
                }
            }
        }
    }
}
