use std::io;

fn read_line_as_numbers() -> Vec<i32> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let value:Vec<i32> = s
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    value
}

fn replace(vec: &mut Vec<i32>, i:usize, j:usize, k:usize) {
    let first_slice = vec.drain(i..k).collect::<Vec<_>>();
    let second_slice = vec.drain(i..=(i + j - k)).collect::<Vec<_>>();
    
    vec.splice(i..i, first_slice);
    vec.splice(i..i, second_slice);
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
    let num = read_line_as_numbers();
    let (n, m) = (num[0], num[1]);
    let mut vec = (1..=n).collect();

    for _ in 0..m {
        let elem = read_line_as_numbers();
        let (i, j, k) = (elem[0] as usize, elem[1] as usize, elem[2] as usize);
        replace(&mut vec, i - 1, j - 1, k - 1);
    }

    print_vector(vec);
}