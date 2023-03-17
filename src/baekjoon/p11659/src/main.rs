use std::io::{ BufWriter, Write, stdout, stdin };

fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn read_line_as_sum() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut sum = 0;
    line.split_whitespace()
        .map(|s| {
            sum += s.parse::<usize>().unwrap();
            sum
        })
        .collect()
}

fn get_sum(i: usize, j: usize, nums: &Vec<usize>) -> usize {
    if i == 0 {
        nums[j]
    } else {
        nums[j] - nums[i - 1]
    }
}

fn main() {
    let args = read_line_as_numbers();
    let (_n, m) = (args[0], args[1]);
    let nums = read_line_as_sum();
    
    let mut out = BufWriter::new(stdout());
    for _ in 0..m {
        let args = read_line_as_numbers();
        let (i, j) = (args[0], args[1]);
        writeln!(out, "{}", get_sum(i - 1, j - 1, &nums)).unwrap();
    }
}