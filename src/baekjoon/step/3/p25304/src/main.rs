use std::io;

fn read_line_as_num() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn read_line_and_mult() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers:Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    numbers.into_iter().reduce(|res, v| res * v).unwrap()
}

fn main() {
    let total_count = read_line_as_num();
    let loop_num = read_line_as_num();

    let mut sum = 0;
    for _ in 0..loop_num {
        sum = sum + read_line_and_mult();
    }

    if total_count == sum {
        println!("Yes");
    } else {
        println!("No");
    }
}
