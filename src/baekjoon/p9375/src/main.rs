use std::collections::HashMap;

fn read_line_as_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn read_line_as_strings() -> Vec<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().split_whitespace().map(|s| s.to_string()).collect()
}

fn main() {
    let n = read_line_as_number();
    for _ in 0..n {
        let count = read_line_as_number();
        let mut map = HashMap::new();
        for _ in 0..count {
            let s = read_line_as_strings();
            let target = String::from(&s[1]);
            map
                .entry(target)
                .and_modify(|v| *v = *v + 1)
                .or_insert(1);
        }
        
        let result = map
            .iter()
            .fold(1, |mult, (_, v)| mult * (v + 1)) - 1;
        
        println!("{}", result);
    }
}
