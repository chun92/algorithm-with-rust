use std::io;
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
    loop {
        let number = read_line_as_numbers();
        match (number[0], number[1]) {
            (0, 0) => break,
            (x, y) => println!("{}", x + y)
        }
    }
}
