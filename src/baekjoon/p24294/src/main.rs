fn read_line_as_number() -> usize {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

fn main() {
    let (w1, h1, w2, h2) =
        (read_line_as_number(), read_line_as_number(), read_line_as_number(), read_line_as_number());
    println!("{}", w1 + h1 * 2 + w2 + h2 * 2 + w1.abs_diff(w2) + 4);
}
