use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let mut handle = stdin.lock();

    let mut str = String::new();
    handle.read_line(&mut str).unwrap();
    let args = str
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let (_n, m) = (args[0], args[1]);


    let mut sum: u64 = 0;
    let mut vec = vec![0; m];
    vec[0] += 1;

    let mut buf = String::new();
    handle.read_line(&mut buf).unwrap();
    let m = m as u64;
    buf
        .trim()
        .split_whitespace()
        .for_each(|s| {
            sum = (sum + s.parse::<u64>().unwrap()) % m as u64;
            vec[sum as usize] += 1;
        });

    let mut result: u64 = 0;

    for i in 0..m as usize {
        if vec[i] > 1 {
            result += (vec[i] * (vec[i] - 1)) / 2;
        }
    }

    println!("{}", result);
}
