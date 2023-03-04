use std::io;
fn read_line_as_numbers() -> Vec<i32> {
    let mut s = String::new();
    let bytes_read = io::stdin().read_line(&mut s).unwrap();

    if bytes_read == 0 {
        return vec![];
    }
    
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
        if number.len() == 0 {
            break;
        }
        println!("{}", number[0] + number[1]);
    }
}
