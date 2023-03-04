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

fn print_fraction(numerator: i32, denominator: i32) {
    let gcd = get_gcd(numerator, denominator);
    println!("{}/{}", numerator / gcd, denominator / gcd);
}

fn main() {
    let n = read_line_as_numbers()[0];
    let numbers = read_line_as_numbers();
    let base = numbers[0];
    for i in 1..n {
        print_fraction(base, numbers[i as usize]);
    }
}
