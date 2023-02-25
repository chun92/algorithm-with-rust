use std::io;

fn read_line_as_number() -> i32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let value = s.trim().parse::<i32>().unwrap();
    value
}

fn read_line_as_number_vec() -> Vec<i32> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let result:Vec<i32> = s.trim().chars().map(|x| {
        x.to_digit(10).unwrap() as i32
    }).collect::<Vec<i32>>();

    result
}

fn main() {
    let _n = read_line_as_number();
    let num:i32 = read_line_as_number_vec().iter().sum();

    println!("{}", num);
}
