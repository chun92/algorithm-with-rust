use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::io::{stdin, stdout, BufWriter, Write};

fn read_line_as_number() -> i32 {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let n = read_line_as_number();
    let mut heap = BinaryHeap::new();
    let mut out = BufWriter::new(stdout());

    for _ in 0..n {
        let x = read_line_as_number();
        if x == 0 {
            let result = heap.pop();
            match result {
                Some(Reverse(x)) => writeln!(out, "{}", x).unwrap(),
                None => writeln!(out, "0").unwrap(),
            }
        } else {
            heap.push(Reverse(x));
        }
    }
}
