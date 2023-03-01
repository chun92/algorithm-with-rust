use std::{num::ParseIntError};

fn read_line_as_number() -> Result<i32, ParseIntError> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>()
}

fn check_perfect_number(num: i32) {
    let range = {
        if num % 2 == 0 {
            num / 2
        } else {
            num / 3
        }
    };

    let mut list: Vec<i32> = Vec::new();

    let mut sum = 0;
    for i in 1..=range {
        if num % i == 0 {
            sum += i;
            list.push(i);
        }
    }

    if sum == num {
        print!("{} = ", num);
        list.iter().enumerate().for_each(|(i, &x)| {
            if i == list.len() - 1 {
                print!("{}", x);
            } else {
                print!("{} + ", x);
            }
        });
        println!();
    } else {
        println!("{} is NOT perfect.", num);
    }
}

fn main() {
    while let Ok(num) = read_line_as_number() {
        if num == -1 {
            break;
        }
        check_perfect_number(num);
    }
}
