fn read_line_as_numbers() -> Vec<u64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let num1 = read_line_as_numbers();
    let num2 = read_line_as_numbers();

    let (p1, q1) = (num1[0], num1[1]);
    let (p2, q2) = (num2[0], num2[1]);
    let gcd1 = gcd(p1, q1);
    let gcd2 = gcd(p2, q2);
    
    let (p1, q1) = (p1 / gcd1, q1 / gcd1);
    let (p2, q2) = (p2 / gcd2, q2 / gcd2);
    let gcd3 = gcd(q1, q2);
    let (p1_final, q1_final) = (p1 * (q2 / gcd3), q1 * (q2 / gcd3));
    let (p2_final, _q2_final) = (p2 * (q1 / gcd3), q2 * (q1 / gcd3));
    let (p, q) = (p1_final + p2_final, q1_final);
    let gcd4 = gcd(p, q);
    println!("{} {}", p / gcd4, q / gcd4);
}
