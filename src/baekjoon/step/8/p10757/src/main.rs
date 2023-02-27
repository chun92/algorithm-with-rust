const MAX: u64 = 10_000_000_000_000_000_000;
const MAGNITUDE: usize = 19;
fn add_big_numbers(a: u64, b: u64) -> (bool, u64) {
    let sum = a + b;
    let overflow = sum >= MAX;
    if overflow {
        (true, sum - MAX)
    } else {
        (false, sum)
    }
}

fn read_line_as_strings() -> Vec<&str> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace().collect()
}

fn main() {
    let strings = read_line_as_strings();
    let a = strings[0];
    let b = strings[1];

    let a_len = a.len();
    let b_len = b.len();
}
