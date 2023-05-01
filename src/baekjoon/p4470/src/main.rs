fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn main() {
    let n = read_line_as_string().trim().parse::<i32>().unwrap();
    for i in 1..=n {
        let s = read_line_as_string();
        print!("{}. {}", i, s);
    }
}
