fn read_line_as_numbers() -> Vec<u32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    loop {
        let numbers = read_line_as_numbers();
        if numbers[0] == 0 && numbers[1] == 0 {
            break;
        }
        println!("{}", numbers[0] + numbers[1]);
    }
}
