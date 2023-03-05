fn read_line_as_numbers() -> Vec<u32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_binomial_coefficient(n: u32, k: u32) -> u32 {
    let mut result = 1;
    for i in 0..k {
        result *= n - i;
        result /= i + 1;
    }
    result
}

fn main() {
    let n = read_line_as_numbers()[0];
    for _ in 0..n {
        let numbers = read_line_as_numbers();
        let n = numbers[1];
        let k = numbers[0];
        println!("{}", get_binomial_coefficient(n, k));
    }
}
