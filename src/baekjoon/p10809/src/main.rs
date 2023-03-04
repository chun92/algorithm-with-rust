use std::io;

fn print_vector<T: std::fmt::Display>(vec:Vec<T>) {
    for (i, v) in vec.iter().enumerate() {
        if i < vec.len() - 1 {
            print!("{} ", v);
        } else {
            println!("{}", v);
        }
    }
}

fn read_line_as_number_vec() -> Vec<i32> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut result:Vec<i32> = vec![-1; 26];

    for (i, v) in s.trim().chars().enumerate() {
        let index = (v as usize) - ('a' as usize);
        if result[index] == -1 {
            result[index] = i.try_into().unwrap();
        } else {
            result[index] = std::cmp::min(result[index], i.try_into().unwrap());
        }
    }

    result
}

fn main() {
    let result = read_line_as_number_vec();
    print_vector(result);
}
