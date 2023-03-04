use std::io;

fn read_line_as_two_numbers() -> (i32, i32) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let a = iter.next().unwrap().parse::<i32>().unwrap();
    let b = iter.next().unwrap().parse::<i32>().unwrap();
    (a, b)
}

fn main() {
    let (n, k) = read_line_as_two_numbers();

    let range_max = {
        if n % 2 == 0 {
            n / 2
        } else {
            n / 3
        }
    };

    let mut count = 0; 
    for i in 1..=range_max {
        if n % i == 0 {
            count += 1;
        }

        if count == k {
            println!("{}", i);
            return;
        }
    }

    if count == k - 1 {
        println!("{}", n);
        return;
    }

    println!("0");
}
