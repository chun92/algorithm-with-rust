fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut found = false;
    for i in 0..5 {
        let input = read_line_as_string();
        if input.contains("FBI") {
            if !found {
                print!("{}", i + 1);
                found = true;
            } else {
                print!(" {}", i + 1);
            }
        }
    }

    if !found {
        println!("HE GOT AWAY!");
    } else {
        println!();
    }
}
