#[derive(PartialEq)]
enum Paren {
    LParen,
    LBra,
}

fn read_line_as_string() -> Option<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    
    if input.trim_end() == "." {
        None
    } else {
        Some(input.trim_end().to_string())
    }
}

fn main() {
    while let Some(line) = read_line_as_string() {
        let mut stack = Vec::new();
        let result = line
            .chars()
            .fold(true,|acc, c|{
                match c {
                    '(' => {
                        stack.push(Paren::LParen);
                        acc && true
                    },
                    '[' => {
                        stack.push(Paren::LBra);
                        acc && true
                    },
                    ')' => {
                        if stack.pop() != Some(Paren::LParen) {
                            acc && false
                        } else {
                            acc && true
                        }
                    }
                    ']' => {
                        if stack.pop() != Some(Paren::LBra) {
                            acc && false
                        } else {
                            acc && true
                        }
                    }
                    _ => acc && true,
                }
            });

            println!("{}", if result && stack.is_empty() {"yes"} else {"no"});
    }
}
