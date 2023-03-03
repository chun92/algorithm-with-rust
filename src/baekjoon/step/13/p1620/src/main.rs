use std::collections::HashMap;
use std::io::Write;

enum Pokemon {
    Number(i32),
    Name(String),
}

fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn read_line_as_string() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn read_line_as_pokemon() -> Pokemon {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    if let Ok(num) = line.trim().parse() {
        Pokemon::Number(num)
    } else {
        Pokemon::Name(line.trim().to_string())
    }
}

fn main() {
    let arguments = read_line_as_numbers();
    let (n, m) = (arguments[0], arguments[1]);

    let mut index_map = HashMap::new();
    let mut name_map = HashMap::new();
    for i in 1..=n {
        let name = read_line_as_string();
        index_map.insert(i, name.clone());
        name_map.insert(name, i);
    }

    let mut out = std::io::BufWriter::new(std::io::stdout());
    for _ in 0..m {
        match read_line_as_pokemon() {
            Pokemon::Number(num) => writeln!(out, "{}", index_map.get(&num).unwrap()).unwrap(),
            Pokemon::Name(name) => writeln!(out, "{}", name_map.get(&name).unwrap()).unwrap(),
        }
    }
    writeln!(out).unwrap();
}
