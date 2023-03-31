fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let n = read_line_as_string();
    let mut count = 0;
    let mut temp = n;

    while temp.len() > 1 {
        let mut temp_sum = 0;
        temp.chars().for_each(|c| {
            temp_sum += c.to_digit(10).unwrap() as i32;
        });

        count += 1;
        temp = temp_sum.to_string();
    }
    println!("{}", count);
    println!("{}", if temp.parse::<i32>().unwrap() % 3 == 0 { "YES" } else { "NO" });
}
