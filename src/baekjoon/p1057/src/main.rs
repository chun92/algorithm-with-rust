fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (_n, a, b) = {
        let v = read_line_as_numbers();
        (v[0], v[1], v[2])
    };

    let mut count = 1;
    let mut a = a;
    let mut b = b;
    loop {
        if (a + 1) / 2 == (b + 1) / 2 {
            println!("{}", count);
            return;
        }
        a = (a + 1) / 2;
        b = (b + 1) / 2;
        count += 1;
    }
}
