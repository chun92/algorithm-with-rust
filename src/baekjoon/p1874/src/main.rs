use std::io::{BufWriter, Write, stdout};

fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let n = read_line_as_number();

    let mut vec_base = (1..=n).rev().collect::<Vec<i32>>();
    let mut vec_operators = Vec::new();
    let mut vec_result: Vec<i32> = Vec::new();

    for _ in 0..n {
        let num = read_line_as_number();

        if vec_result.last().unwrap_or(&0) > &num {
            println!("NO");
            return;
        } else if vec_result.last().unwrap_or(&0) == &num {
            vec_result.pop();
            vec_operators.push('-');
        } else {
            while vec_base.last().unwrap() != &num {
                vec_result.push(vec_base.pop().unwrap());
                vec_operators.push('+');
            }

            vec_base.pop();
            vec_operators.push('+');
            vec_operators.push('-');
        }
    }

    let mut out = BufWriter::new(stdout());
    for op in vec_operators {
        writeln!(out, "{}", op).unwrap();
    }
}
