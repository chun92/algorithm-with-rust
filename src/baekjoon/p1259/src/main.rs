fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn main() {
    loop {
        let num_as_string = read_line_as_string();
        let str = num_as_string.trim();
        if str.len() == 1 && str == "0" {
            break;
        }

        let result = str
            .chars()
            .zip(str.chars().rev())
            .fold(true, |acc, (a, b)| {
                if a == b {
                    acc
                } else {
                    false
                }
            });
        if result {
            println!("yes");
        } else {
            println!("no");
        }
    }
}
