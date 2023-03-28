fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn main() {
    loop {
        let input = read_line_as_string();
        let input = input.trim();
        if input == "END" {
            break;
        }
        input
            .chars()
            .rev()
            .for_each(|c| print!("{}", c));
        println!();
    }
}