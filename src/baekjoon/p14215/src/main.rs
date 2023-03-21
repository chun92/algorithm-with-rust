fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (a, b, c) = {
        let mut vec = read_line_as_numbers();
        vec.sort_unstable();
        (vec[0], vec[1], vec[2])
    };

    if a + b > c {
        println!("{}", a + b + c);
    } else {
        println!("{}", 2 * (a + b) - 1);
    }
}
