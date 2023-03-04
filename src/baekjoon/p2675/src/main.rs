use std::io;

fn read_line_as_number() -> i32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let value = s.trim().parse::<i32>().unwrap();
    value
}

fn read_line_and_extend_str() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let value:Vec<String> = s.trim().split_whitespace().map(|x| String::from(x)).collect();
    let count = &value[0].parse::<i32>().unwrap();
    let target_str = &value[1];

    target_str.chars().for_each(|x| {
        for _ in 0..*count {
            print!("{}", x);
        }
    });
    
    println!("");
}

fn main() {
    let n = read_line_as_number();
    for _ in 0..n {
        read_line_and_extend_str();
    }
}
