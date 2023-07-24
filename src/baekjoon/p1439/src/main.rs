fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let s = read_line_as_string();
    let mut ans = 0;
    let first = s.chars().nth(0).unwrap();
    let mut current = first;
    for c in s.chars() {
        if current != c && c != first {
            ans += 1;
        }
        current = c;
    }
    println!("{}", ans);
}
