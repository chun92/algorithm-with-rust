fn read_line_as_number() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn check_line_even_odd() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().chars().last().unwrap().to_digit(10).unwrap();
    if n % 2 == 0 {
        "even".to_string()
    } else {
        "odd".to_string()
    }
}

fn main() {
    let n = read_line_as_number();
    for _ in 0..n {
        println!("{}", check_line_even_odd());
    }
}
