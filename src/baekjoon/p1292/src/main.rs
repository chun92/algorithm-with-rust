fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (l, r) = {
        let v = read_line_as_numbers();
        (v[0], v[1])
    };

    let mut vec = vec![0; r + 1];

    let mut index = 1;
    let mut count = 0;
    for i in 1..r + 1 {
        vec[i] = index;
        count += 1;
        if index == count {
            index += 1;
            count = 0;
        }
    }

    let mut ans = 0;
    for i in l..r + 1 {
        ans += vec[i];
    }

    println!("{}", ans);
}
