fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (n, k) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut vec = Vec::new();

    for _ in 0..n {
        let v = read_line_as_numbers();
        vec.push((v[0], v[1], v[2], v[3]));
    }

    vec.sort_by(|(_, a1, a2, a3), (_, b1, b2, b3)| {
        if a1 == b1 {
            if a2 == b2 {
                b3.cmp(a3)
            } else {
                b2.cmp(a2)
            }
        } else {
            b1.cmp(a1)
        }
    });

    let mut rank = 1;
    let mut prev = (0, 0, 0);

    for (i, (a0, a1, a2, a3)) in vec.iter().enumerate() {
        if prev != (*a1, *a2, *a3) {
            rank = i + 1;
        }

        if *a0 == k {
            println!("{}", rank);
            break;
        }

        prev = (*a1, *a2, *a3);
    }
}
