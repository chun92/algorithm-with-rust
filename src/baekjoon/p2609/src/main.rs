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

fn main() {
    let numbers = read_line_as_numbers();
    let (n, m) = (numbers[0], numbers[1]);

    let gcd = get_gcd(n, m);
    let lcm = n * m / gcd;
    println!("{}", gcd);
    println!("{}", lcm);
}
