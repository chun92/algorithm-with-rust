use std::io;

fn read_first_num() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let loop_num = read_first_num();
    let mut sum = 0;
    for i in 1..=loop_num {
        sum = sum + i;
    }
    println!("{}", sum);
}
