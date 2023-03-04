use std::io;
fn read_line_as_number() -> i32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let value = s.trim().parse::<i32>().unwrap();
    return value;
}

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

fn main() {
    let _count = read_line_as_number();
    let numbers = read_line_as_numbers();
    let target = read_line_as_number();

    let mut sum = 0;
    numbers.into_iter().for_each(|x| {
        if x == target { 
            sum = sum + 1;
        }
    });
    println!("{}", sum);
}
