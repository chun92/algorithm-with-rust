fn read_line_as_two_nubmers() -> (i32, i32) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let a = iter.next().unwrap().parse::<i32>().unwrap();
    let b = iter.next().unwrap().parse::<i32>().unwrap();
    (a, b)
}
fn main() {
    loop {
        let (x, y) = read_line_as_two_nubmers();
        match (x, y) {
            (0, 0) => break,
            (x, y) if x > y && x % y == 0 => println!("multiple"),
            (x, y) if x < y && y % x == 0 => println!("factor"),
            _ => println!("neither"),
        }
    }
}
