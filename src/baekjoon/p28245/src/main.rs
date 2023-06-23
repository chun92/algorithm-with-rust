use std::io::{stdin, stdout, BufWriter, Write, Stdout};

fn read_line_as_number() -> u64 {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please type a number")
}

fn find_max_idx(n: u64) -> u64 {
    let mut idx = 0;
    let mut n = n;
    while n > 0 {
        idx += 1;
        n = n >> 1;
    }
    idx
}

fn solve(n: u64, output: &mut BufWriter<Stdout>) {
    if n == 0 {
        writeln!(output, "0 0").unwrap();
        return;
    }

    let max_idx = find_max_idx(n);
    let mut min_diff = u64::MAX;
    let mut min_m = u64::MAX;
    let mut min_pair = (0, 0);
    for i in 0..max_idx {
        for j in i..max_idx {
            let m = (1 << i) + (1 << j);
            if n == m {
                writeln!(output, "{} {}", i, j).unwrap();
                return;
            } else {
                let diff = n.abs_diff(m);
                if diff < min_diff || (diff == min_diff && m < min_m) {
                    min_diff = diff;
                    min_m = m;
                    min_pair = (i, j);
                }
            }
        }
    }
    writeln!(output, "{} {}", min_pair.0, min_pair.1).unwrap();
}

fn main() {
    let t = read_line_as_number();
    let mut output = BufWriter::new(stdout());
    for _ in 0..t {
        let n = read_line_as_number();
        solve(n, &mut output);
    }
}
