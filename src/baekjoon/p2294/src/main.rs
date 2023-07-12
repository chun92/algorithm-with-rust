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
        let v = read_line_as_numbers()[0];
        vec.push(v);
    }

    let mut memo = vec![usize::MAX; k + 1];
    memo[0] = 0;

    for d in &vec {
        if *d > k {
            continue;
        }
        memo[*d] = 1;
    }

    for i in 1..=k {
        if memo[i] == usize::MAX {
            continue;
        }

        for d in &vec {
            if i + d > k {
                continue;
            }
            memo[i + d] = std::cmp::min(memo[i + d], memo[i] + 1);
        }
    }

    let result = memo[k];

    if result == usize::MAX {
        println!("-1");
    } else {
        println!("{}", result);
    }
}
