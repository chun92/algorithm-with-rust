fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    loop {        
        let (a, b, c) = {
            let vec = read_line_as_numbers();
            (vec[0], vec[1], vec[2])
        };

        match (a, b, c) {
            (0, 0, 0) => break,
            (a, b, c) if c >= a + b || a >= b + c || b >= c + a => println!("Invalid"),
            (a, b, c) if a == b && b == c => println!("Equilateral"),
            (a, b, c) if a == b || b == c || c == a => println!("Isosceles"),
            _ => println!("Scalene"),
        }
    }
}
