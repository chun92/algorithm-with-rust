fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let _n = read_line_as_string().parse::<i32>().unwrap();
    let str = read_line_as_string();
    let result = str
        .chars()
        .fold(0, |acc, c| 
            match c {
                'a' | 'i' | 'u' | 'e' | 'o' => acc + 1,
                _ => acc
            }
        );
    println!("{}", result);
}
