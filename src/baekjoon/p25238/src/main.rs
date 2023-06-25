fn read_line_as_numbers() -> Vec<u32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

fn main() {
    let (a, b) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let result = a as f64 / 100 as f64 * (100 - b) as f64;
    println!("{}", if result >= 100.0 { 0 } else { 1 });
}
