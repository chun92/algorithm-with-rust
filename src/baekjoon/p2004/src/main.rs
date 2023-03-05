fn read_line_as_numbers() -> Vec<u32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
fn get_one(n: u32, x: u32) -> u32 {
    let mut count = 0;
    let mut i = n;
    while i > 0 {
        count += i / x;
        i /= x;
    }
    count
}

fn get(n: u32) -> (u32, u32) {
    (get_one(n, 2), get_one(n, 5))
}

fn get_result(n: u32, m: u32) -> (u32, u32) {
    if n == m || m == 0 {
        return (0, 0);
    }
    let (n2, n5) = get(n);
    let (m2, m5) = get(m);
    let (n_m2, n_m5) = get(n - m);
    (n2 - m2 - n_m2, n5 - m5 - n_m5)
}

fn main() {
    let numbers = read_line_as_numbers();
    let (n, m) = (numbers[0], numbers[1]);
    let (two, five) = get_result(n, m);
    println!("{}", std::cmp::min(two, five));
}
