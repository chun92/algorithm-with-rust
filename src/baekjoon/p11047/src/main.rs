fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let args = read_line_as_numbers();
    let (n, k) = (args[0], args[1]);

    let mut coins = Vec::new();

    for _ in 0..n {
        let num = read_line_as_numbers()[0];
        coins.push(num);
    }

    let mut remain = k;
    let mut count = 0;
    coins
        .iter()
        .rev()
        .for_each(|&coin| {
            let c = remain / coin;
            remain -= coin * c;
            count += c;
        });
    
    println!("{}", count);
}