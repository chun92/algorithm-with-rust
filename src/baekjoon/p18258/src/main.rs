use std::{io::{ Write, BufWriter, BufRead, stdout, stdin }, collections::VecDeque};


fn main() {
    let mut lines = stdin().lock().lines();
    let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let mut out = BufWriter::new(stdout());

    let mut queue = VecDeque::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let words = line
            .split_whitespace()
            .collect::<Vec<&str>>();

        match words[0] {
            "push" => {
                queue.push_back(words[1].parse::<i32>().unwrap());
            },
            "pop" => {
                if queue.is_empty() {
                    writeln!(out, "-1").unwrap();
                } else {
                    writeln!(out, "{}", queue.pop_front().unwrap()).unwrap();
                }
            },
            "size" => {
                writeln!(out, "{}", queue.len()).unwrap();
            },
            "empty" => {
                if queue.is_empty() {
                    writeln!(out, "1").unwrap();
                } else {
                    writeln!(out, "0").unwrap();
                }
            },
            "front" => {
                if queue.is_empty() {
                    writeln!(out, "-1").unwrap();
                } else {
                    writeln!(out, "{}", queue.front().unwrap()).unwrap();
                }
            },
            "back" => {
                if queue.is_empty() {
                    writeln!(out, "-1").unwrap();
                } else {
                    writeln!(out, "{}", queue.back().unwrap()).unwrap();
                }
            },
            _ => { panic!(); }
        }
    }
}
