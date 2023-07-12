fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let str = read_line_as_string();

    let mut bars = Vec::new();
    let mut previous_char = ' ';
    let mut total = 0;
    for c in str.chars() {
        match c {
            '(' => {
                if previous_char == '(' {
                    bars.push(1);
                }
            },
            ')' => {
                if previous_char == ')' {
                    total += bars.pop().unwrap();
                } else {
                    bars
                        .iter_mut()
                        .for_each(|x| *x += 1);
                }
            },
            _ => panic!("Invalid character: {}", c)
        }

        previous_char = c;
    }

    println!("{}", total);
}
