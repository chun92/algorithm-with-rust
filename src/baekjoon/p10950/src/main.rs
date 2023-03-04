use std::io;

fn read_first_num() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn read_line_and_add() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers:Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    numbers.iter().sum()
}

fn main() {
    let loop_num = read_first_num();
    for _ in 0..loop_num {
        let result = read_line_and_add();
        println!("{}", result);
    }
}
