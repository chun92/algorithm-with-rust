fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn fibonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n -1 ) + fibonacci(n - 2);
    }
}

fn main() {
    let n = read_line_as_number();
    let result = fibonacci(n);
    println!("{}", result);
}
