fn read_line_as_number() -> i64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i64>().unwrap()
}

fn main() {
    let n = read_line_as_number();
    let mut sum = 0;
    for i in 1..=n - 2 {
        // let a = n * (n - 1 - i);
        // let b = n * (n - 1) / 2;
        // let c = i * (i + 1)/ 2 ;
        // sum += a - (b - c);
        sum += n * n / 2 - n / 2 + i * (i + 1) / 2 - n * i ;
    }
    println!("{}", sum);
    println!("3");
}
