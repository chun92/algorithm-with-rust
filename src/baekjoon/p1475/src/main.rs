fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let str = read_line_as_string();
    let mut numbers = vec![0; 10];
    for c in str.chars() {
        let n = c.to_digit(10).unwrap() as usize;
        numbers[n] += 1;
    }

    let sum_of_6_9 = numbers[6] + numbers[9];
    numbers[6] = sum_of_6_9 / 2 + sum_of_6_9 % 2;
    numbers[9] = sum_of_6_9 / 2;
    let max = numbers.iter().max().unwrap();
    println!("{}", max);
}
