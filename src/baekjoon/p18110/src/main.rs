use std::io::{self, stdin, BufRead};

fn main() {
    let stdin = stdin();
    let mut reader = io::BufReader::new(stdin.lock());

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();

    let n = input.trim().parse::<usize>().unwrap();

    let mut vec = Vec::new();
    for _ in 0..n {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let num = input.trim().parse::<u32>().unwrap();
        vec.push(num);
    }

    vec.sort_unstable();
    let cut = (n as f64 / 100.0 * 15.0).round() as usize;
    let result: Vec<u32> = vec[cut..(n - cut)].to_vec();
    let result = (result.iter().sum::<u32>() as f64 / result.len() as f64).round() as usize;
    println!("{:?}", result);
}
