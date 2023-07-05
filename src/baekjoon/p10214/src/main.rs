fn read_lien_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let test_cases = read_lien_as_numbers()[0];

    for _ in 0..test_cases {
        let (mut a, mut b) = (0, 0);
        for _ in 0..9 {
            let (x, y) = {
                let v = read_lien_as_numbers();
                (v[0], v[1])
            };
            a += x;
            b += y;
        }

        if a > b {
            println!("Yonsei");
        } else if a < b {
            println!("Korea");
        } else {
            println!("Draw");
        }
    }
}
