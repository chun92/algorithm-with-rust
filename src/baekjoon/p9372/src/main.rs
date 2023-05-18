fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
fn main() {
    let test_cases = read_line_as_numbers()[0];

    for _ in 0..test_cases {
        let (n, m) = {
            let args = read_line_as_numbers();
            (args[0], args[1])
        };

        for _ in 0..m {
            let _args = read_line_as_numbers();
        }

        println!("{}", n - 1);
    }
}
