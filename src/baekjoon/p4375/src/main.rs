fn read_line_as_number() -> Option<i32> {
    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text).unwrap();
    let trimmed = input_text.trim();
    trimmed.parse::<i32>().ok()
}

fn main() {
    while let Some(n) = read_line_as_number() {
        let mut target = 1;
        let mut temp = n;
        let mut result = 1;
        while temp > 0 {
            target = target * 10 + 1;
            temp /= 10;
            result += 1;
        }

        if target / 10 >= n {
            target /= 10;
            result -= 1;
        }

        target %= n;
        while target != 0 {
            target = target * 10 + 1;
            target %= n;
            result += 1;
        }

        println!("{}", result);
    }
}
