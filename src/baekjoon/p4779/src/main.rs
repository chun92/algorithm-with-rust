fn read_line_as_number() -> Option<i32> {
    let mut input = String::new();
    let n = std::io::stdin().read_line(&mut input).unwrap();
    if n > 0 {
        input.trim().parse::<i32>().ok()
    } else {
        None
    }
}

fn cantor(n: i32) -> String {
    if n == 0 {
        return "-".to_string();
    }
    let num = 3_i32.pow(n as u32 - 1);
    let empty_str = " ".repeat(num as usize);
    cantor(n - 1) + &empty_str + &cantor(n - 1)
}

fn main() {
    while let Some(n) = read_line_as_number() {
        println!("{}", cantor(n));
    }
}
