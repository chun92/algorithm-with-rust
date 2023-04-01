fn read_line_as_numbers() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let (n, k) = {
        let i = read_line_as_numbers();
        (i[0], i[1])
    };

    let mut vec = vec![n];
    let mut current = 0;
    let mut hash_map = std::collections::HashMap::new();
    loop {
        let mut next = Vec::new();
        for i in vec.iter() {
            if *i == k {
                println!("{}", current);
                return;
            }
            if *i > k {
                if hash_map.get(&(*i - 1)).is_none() {
                    next.push(*i - 1);
                    hash_map.insert(*i - 1, true);
                }
            } else {
                if hash_map.get(&(*i - 1)).is_none() {
                    next.push(*i - 1);
                    hash_map.insert(*i - 1, true);
                }
                if hash_map.get(&(*i * 2)).is_none() {
                    next.push(*i * 2);
                    hash_map.insert(*i * 2, true);
                }
                if hash_map.get(&(*i + 1)).is_none() {
                    next.push(*i + 1);
                    hash_map.insert(*i + 1, true);
                }
            }
        }
        current += 1;
        vec = next;
    }
}
