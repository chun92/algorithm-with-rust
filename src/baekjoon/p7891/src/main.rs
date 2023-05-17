fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_numbers()[0];

    for _ in 0..n {
        let (a, b) = {
            let args = read_line_as_numbers();
            (args[0], args[1])
        };

        println!("{}", a + b);
    }
}
