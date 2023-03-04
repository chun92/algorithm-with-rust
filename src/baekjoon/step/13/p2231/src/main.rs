fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn get_generate_sum(num: i32) -> i32 {
    let mut sum = num;

    let input = num.to_string();
    for c in input.chars() {
        if c.is_digit(10) {
            sum += c.to_digit(10).unwrap() as i32;
        }
    }
    sum
}

fn main() {
    let n = read_line_as_number();
    let mut result = 0;
    for i in 0..n {
        let sum = get_generate_sum(i);
        if sum == n {
            result = i;
            break;
        }
    }
    println!("{}", result);
}
