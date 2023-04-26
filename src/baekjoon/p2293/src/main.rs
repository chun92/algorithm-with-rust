fn read_line_as_numbers() -> Vec<u32> {
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

    let mut coins = Vec::new();
    for _ in 0..n {
        coins.push(read_line_as_numbers()[0]);
    }

    let mut memo = vec![0; k as usize + 1];

    for coins in coins {
        for i in 1..=k {
            if i < coins {
                continue;
            }
            if i == coins {
                memo[i as usize] += 1;
            } else {
                memo[i as usize] += memo[(i - coins) as usize];
            }
        }
    }

    println!("{}", memo[k as usize]);
}
