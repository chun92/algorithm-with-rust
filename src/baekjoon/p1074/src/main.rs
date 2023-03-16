fn read_line_as_numbers() -> Vec<u64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn calc(n: u64, x: u64, y: u64) -> u64 {
    if n == 0 {
        match (x, y) {
            (0, 0) => 0,
            (0, 1) => 1,
            (1, 0) => 2,
            (1, 1) => 3,
            _ => unreachable!(),
        }
    } else {
        let half = 2u64.pow(n as u32 - 1);
        match (x < half, y < half) {
            (true, true) => calc(n - 1, x, y),
            (true, false) => half * half + calc(n - 1, x, y - half),
            (false, true) => 2 * half * half + calc(n - 1, x - half, y),
            (false, false) => 3 * half * half + calc(n - 1, x - half, y - half),
        }
    }
}

fn main() {
    let args = read_line_as_numbers();
    let (n, r, c) = (args[0], args[1], args[2]);
    let result = calc(n, r, c);
    println!("{}", result);
}