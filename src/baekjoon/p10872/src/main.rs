fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn factorial(n: i32) -> i32 {
    if n == 0 {
        return 1;
    }
    n * factorial(n - 1)
}

fn main() {
    let n = read_line_as_number();
    let result = factorial(n);
    println!("{}", result);
}
