use std::io::{stdin, stdout, BufWriter, Write};

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn dp(s: usize, e: usize, numbers: &Vec<usize>, memo: &mut Vec<Vec<i32>>) -> i32 {
    if s > e {
        return 1;
    }

    if s == e {
        return 1;
    }

    if memo[s][e] != -1 {
        return memo[s][e];
    }

    if numbers[s] == numbers[e] {
        memo[s][e] = dp(s + 1, e - 1, numbers, memo);
    } else {
        memo[s][e] = 0;
    }

    memo[s][e]
}

fn main() {
    let n = read_line_as_numbers()[0];
    let numbers = read_line_as_numbers();
    let m = read_line_as_numbers()[0];

    let mut memo = vec![vec![-1; n]; n];

    let mut output = BufWriter::new(stdout());
    for _ in 0..m {
        let (s, e) = {
            let args = read_line_as_numbers();
            (args[0], args[1])
        };

        let result = dp(s - 1, e - 1, &numbers, &mut memo);
        writeln!(output, "{}", if result == 1 { 1 } else { 0 }).unwrap();
    }
}
