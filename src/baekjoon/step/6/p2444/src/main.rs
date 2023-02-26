use std::io;

fn read_line_as_number() -> i32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let value = s.trim().parse::<i32>().unwrap();
    value
}

fn main() {
    let num = read_line_as_number();
    let limit = 2 * num;
    for i in 1..limit {
        for j in 1..limit {
            if (j - num).abs() < (num - (num - i).abs()) {
                print!("*");
            } else if j < num {
                print!(" ");
            } else {
                break;
            }
        }
        println!("");
    }
}