fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let mut index = 0;
    let mut max = 0;
    
    for i in 0..5 {
        let numbers = read_line_as_numbers();
        let sum = numbers.iter().sum();
        if sum > max {
            max = sum;
            index = i + 1;
        }
    }

    println!("{} {}", index, max);
}
