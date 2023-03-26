fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    let size = std::io::stdin().read_line(&mut line).unwrap();
    if size == 0 {
        vec![]
    } else {
        line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect()
    }
}

fn main() {
    loop {
        let vec = read_line_as_numbers();
        if vec.is_empty() {
            break;
        }
        let (n, s) = (vec[0], vec[1]);
        println!("{}", s / (n + 1));
    }
}
