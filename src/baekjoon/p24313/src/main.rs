fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let arguments = read_line_as_numbers();
    let (a1, a0) = (arguments[0], arguments[1]);
    let c = read_line_as_numbers()[0];
    let n0 = read_line_as_numbers()[0];


    if c == a1 {
        if a0 <= 0 {
            println!("1");
        } else {
            println!("0");
        }
    } else if c < a1 {
        println!("0");
    } else if n0 >= a0 / (c - a1) {
        println!("1");
    } else {
        println!("0");
    }
}
