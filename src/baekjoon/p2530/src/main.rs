fn read_line_as_numbers() -> Vec<u32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (h, m, s) = {
        let v = read_line_as_numbers();
        (v[0], v[1], v[2])
    };
    let t = read_line_as_numbers()[0];
    let time = (s + m * 60 + h * 60 * 60 + t) % (24 * 60 * 60);
    println!("{} {} {}", time / 3600, time / 60 % 60, time % 60);
}
