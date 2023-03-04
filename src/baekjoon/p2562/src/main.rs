use std::io;
fn read_line_as_number() -> i32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let value = s.trim().parse::<i32>().unwrap();
    value
}

fn main() {
    let (mut index, mut max) = (0, 0);
    for i in 1..=9 {
        let number = read_line_as_number();
        if max < number {
            max = number;
            index = i;
        }
    }
    println!("{}", max);
    println!("{}", index);
}
