use std::io;

fn read_line_as_numbers() -> Vec<i32> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    
    let values:Vec<i32> = s
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    values
}

fn print_vector<T: std::fmt::Display>(vec:Vec<T>) {
    for (i, v) in vec.iter().enumerate() {
        if i < vec.len() - 1 {
            print!("{} ", v);
        } else {
            println!("{}", v);
        }
    }
}

fn main() {
    let basis = vec![1, 1, 2, 2, 2, 8];
    let numbers = read_line_as_numbers();
    let results:Vec<i32> = basis
        .iter()
        .zip(numbers.iter())
        .map(|(x, y)| {
            x - y
        }).collect();
    print_vector(results);
}
