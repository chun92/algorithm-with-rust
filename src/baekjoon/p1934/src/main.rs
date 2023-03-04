fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        get_gcd(b, a % b)
    }
}

fn get_lcm(a: i32, b: i32) -> i32 {
    a * b / get_gcd(a, b)
}
fn main() {
    let n = read_line_as_numbers()[0];
    for _ in 0..n {
        let numbers = read_line_as_numbers();
        println!("{}", get_lcm(numbers[0], numbers[1]));
    }
}
