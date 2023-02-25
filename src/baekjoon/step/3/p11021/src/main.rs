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
    let count = read_line_as_number();
    for i in 1..=count {
        let number = read_line_as_numbers();
        println!("Case #{}: {}", i, number[0] + number[1]);
    }
}
