fn read_line_from_stdin() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    loop {
        let input = read_line_from_stdin();
        if input.len() == 1 && input.chars().next().unwrap() == '#' {
            break;
        }
        let num = input
            .chars()
            .fold(0, |acc, c| acc + match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => 1,
                _ => 0,
            });
        println!("{}", num);
    }
}
