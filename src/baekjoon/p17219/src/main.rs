use std::collections::{HashMap};

fn read_line_as_strings() -> Vec<String> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace().map(|s| s.to_string()).collect()
}

fn main() {
    let (n, m) = {
        let i = read_line_as_strings();
        (i[0].parse::<i32>().unwrap(), i[1].parse::<i32>().unwrap())
    };

    let mut hash_map = HashMap::new();
    for _ in 0..n {
        let i = read_line_as_strings();
        hash_map.insert(i[0].clone(), i[1].clone());
    }
    for _ in 0..m {
        let i = read_line_as_strings();
        match hash_map.get(&i[0]) {
            Some(v) => println!("{}", v),
            None => println!("Not found"),
        }
    }
}
