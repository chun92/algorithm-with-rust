use std::io;
use std::collections::HashMap;

fn read_line_and_get_most_used_character() -> char {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut result = HashMap::new();
    s.trim().to_uppercase().chars().for_each(|x| {
        if result.contains_key(&x) {
            result.insert(x, result.get(&x).unwrap() + 1);
        } else {
            result.insert(x, 1);
        }
    });

    let max_num = result.iter().map(|(_, v)| v).max().unwrap();
    let result:Vec<char> = result.iter().filter(|(_, v)| *v == max_num).map(|(k, _)| *k).collect();
    if result.len() > 1 {
        '?'
    } else {
        result[0]
    }
}

fn main() {
    let result = read_line_and_get_most_used_character();
    println!("{}", result);
}
