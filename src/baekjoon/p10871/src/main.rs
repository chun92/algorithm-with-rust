use std::io;

fn read_line_as_numbers() -> Vec<i32> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    
    let values:Vec<i32> = s
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    return values;
}

fn print_result(vec: Vec<i32>) {
    for (i, v) in vec.iter().enumerate() {
        if i < vec.len() - 1 {
            print!("{} ", v);
        } else {
            println!("{}", v);
        }
    }
}

fn main() {
    let basis = read_line_as_numbers()[1];
    let numbers = read_line_as_numbers();
    let results = numbers.into_iter().filter(|x| *x < basis).collect::<Vec<_>>();
    print_result(results);
}
