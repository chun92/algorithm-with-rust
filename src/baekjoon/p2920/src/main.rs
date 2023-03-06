fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let nums = read_line_as_numbers();

    let mut inc = 1;
    let mut dec = 8;
    nums
        .iter()
        .for_each(|n| {
            if *n == inc {
                inc += 1;
            }
            if *n == dec {
                dec -= 1;
            }
        });
    
    match (inc, dec) {
        (9, _) => println!("ascending"),
        (_, 0) => println!("descending"),
        _ => println!("mixed"),
    }
}