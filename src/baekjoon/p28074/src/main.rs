fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let str = read_line_as_string();
    let mut m = false;
    let mut o = false;
    let mut b = false;
    let mut i = false;
    let mut s = false;

    str
        .chars()
        .for_each(|c| match c {
            'M' => m = true,
            'O' => o = true,
            'B' => b = true,
            'I' => i = true,
            'S' => s = true,
            _ => (),
        });
    println!("{}", if m && o && b && i && s { "YES" } else { "NO" });
}
