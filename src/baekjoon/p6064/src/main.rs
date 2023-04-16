use std::io::{stdin, stdout, Write, BufWriter};

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn get_lcm(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let n = read_line_as_numbers()[0];

    let mut output = BufWriter::new(stdout());

    for _ in 0..n {
        let numbers = read_line_as_numbers();
        let (m, n) = (numbers[0], numbers[1]);
        let (x, y) = (numbers[2], numbers[3]);

        if x.abs_diff(y) % get_lcm(m, n) != 0 {
            writeln!(output, "-1").unwrap();
            continue;
        }

        let mut x = x;
        let mut y = y;
        loop {
            if x == y {
                writeln!(output, "{}", x).unwrap();
                break;
            } else if x < y {
                x += m;
            } else {
                y += n;
            }
        }
    }
}
