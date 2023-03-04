use std::io;
fn read_line_as_number() -> i32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let value = s.trim().parse::<i32>().unwrap();
    value
}

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

fn get_min_max(vec:Vec<i32>) -> (i32, i32) {
    let mut min = 1_000_000;
    let mut max = -1_000_000;

    vec.into_iter().for_each(|x| {
        if x < min {
            min = x;
        }

        if x > max {
            max = x;
        }
    });
    (min, max)
}

fn main() {
    let _count = read_line_as_number();
    let numbers = read_line_as_numbers();
    let (min, max) = get_min_max(numbers);
    println!("{} {}", min, max);
}
