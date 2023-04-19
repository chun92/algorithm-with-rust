fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    for _ in 0..3 {
        let numbers = read_line_as_numbers();
        let sum = numbers.iter().sum::<i32>();
        match sum {
            0 => println!("D"),
            1 => println!("C"),
            2 => println!("B"),
            3 => println!("A"),
            4 => println!("E"),
            _ => unreachable!(),
        }
    }
}
