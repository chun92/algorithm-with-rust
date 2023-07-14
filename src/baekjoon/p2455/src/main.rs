fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let mut sum = 0;
    let mut max = 0;

    for _ in 0..4 { 
        let (down, up) = {
            let numbers = read_line_as_numbers();
            (numbers[0], numbers[1])
        };
        sum -= down;
        sum += up;
        
        max = max.max(sum);
    }

    println!("{}", max);
}
