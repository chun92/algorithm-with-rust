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

fn print_i32_vector<T: std::fmt::Display>(vec:Vec<T>) {
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

    let mut results: Vec<usize> = vec![0; n as usize];

    for _ in 0..m {
        let numbers = read_line_as_numbers();
        let (i, j, k) = (numbers[0] as usize, numbers[1] as usize, numbers[2] as usize);

        for x in i..=j {
            results[x - 1] = k;
        }
    }

    print_i32_vector(results);
}
