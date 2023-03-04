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
    let argues = read_line_as_numbers();
    let (n, m) = (argues[0], argues[1]);

    let mut results: Vec<i32> = (1..=n).collect();

    for _ in 0..m {
        let numbers = read_line_as_numbers();
        let (i, j) = (numbers[0] as usize, numbers[1] as usize);

        let a = results[i - 1];
        let b = results[j - 1];
        results[i - 1] = b;
        results[j - 1] = a;
    }

    print_vector(results);
}
