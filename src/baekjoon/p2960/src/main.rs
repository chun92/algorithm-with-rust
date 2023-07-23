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

    let mut chieve = vec![false; n + 1];
    let mut count = 0;

    for i in 2..=n {
        if chieve[i] {
            continue;
        }
        count += 1;
        if count == k {
            println!("{}", i);
            return;
        }

        let mut j = i;
        while j <= n {
            if chieve[j] {
                j += i;
                continue;
            }
            if j != i {
                count += 1;
                if count == k {
                    println!("{}", j);
                    return;
                }
            }
            chieve[j] = true;
            j += i;
        }
    }
}
