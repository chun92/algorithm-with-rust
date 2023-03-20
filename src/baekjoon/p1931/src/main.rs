fn read_line_as_numbers() -> Vec<usize> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let n = read_line_as_numbers()[0];

    let mut vec = Vec::new();
    for _ in 0..n {
        let nums = read_line_as_numbers();
        vec.push((nums[0], nums[1]));
    }
    
    vec.sort_by(|a, b| 
        a.0.cmp(&b.0)
    );
    
    vec.sort_by(|a, b| 
        a.1.cmp(&b.1)
    );

    let mut count = 0;
    let mut cur = 0;
    for (x, y) in vec {
        if cur <= x {
            count += 1;
            cur = y;
        }
    }

    println!("{}", count);
}
