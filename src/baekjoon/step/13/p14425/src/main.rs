use std::collections::HashSet;

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
    line
}

fn main() {
    let arguments = read_line_as_numbers();
    let (n, m) = (arguments[0], arguments[1]);

    let mut set: HashSet<String> = HashSet::new();
    for _ in 0..n {
        set.insert(read_line_as_string());
    }

    let mut count = 0;
    for _ in 0..m {
        if set.contains(&read_line_as_string()) {
            count += 1;
        }
    }

    println!("{}", count);
}
