fn read_line_as_number() -> Vec<u32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let mut n = 1000 - read_line_as_number()[0];
    let mut count = 0;
    
    if n >= 500 {
        count += n / 500;
        n %= 500;
    }
    if n >= 100 {
        count += n / 100;
        n %= 100;
    }
    if n >= 50 {
        count += n / 50;
        n %= 50;
    }
    if n >= 10 {
        count += n / 10;
        n %= 10;
    }
    if n >= 5 {
        count += n / 5;
        n %= 5;
    }
    if n >= 1 {
        count += n / 1;
        n %= 1;
    }
    println!("{}", count);
}
