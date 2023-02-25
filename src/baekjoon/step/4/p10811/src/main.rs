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
    let arguements = read_line_as_numbers();
    let (n, m) = (arguements[0], arguements[1] as usize);
    let mut vec: Vec<i32> = (1..=n).collect();

    for _ in 0..m {
        let elem = read_line_as_numbers();
        let (i, j) = (elem[0] as usize, elem[1] as usize);
        
        vec[i-1..=j-1].reverse();
    }

    print_vector(vec);
}
